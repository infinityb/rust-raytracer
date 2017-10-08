use num_cpus;
use light::Light;
use ::compositor::{ColorRGBA, Surface, SurfaceFactory};
use ::{Ray};
use geometry::Intersection;
use scene::{Camera, Scene};
use ::util::Vec3;
use rand::{thread_rng, Rng, Isaac64Rng};

pub static EPSILON: f64 = ::std::f64::EPSILON * 10000.0;

#[derive(Clone, Copy)]
pub struct RenderOptions {
    pub reflect_depth: u32,  // Maximum reflection recursions.
    pub refract_depth: u32,  // Maximum refraction recursions. A sphere takes up 2 recursions.
    pub shadow_samples: u32, // Number of samples for soft shadows and area lights.
    pub gloss_samples: u32,  // Number of samples for glossy reflections.
    pub pixel_samples: u32,  // The square of this is the number of samples per pixel.
}

#[derive(Clone)]
pub struct Renderer {
    pub tasks: Option<usize>, // Minimum number of tasks to spawn.
    pub options: RenderOptions,
}

impl Renderer {
    pub fn render(&self, camera: &Camera, scene: &Scene) -> Surface {
        use rayon::{Configuration, ThreadPool};
        use std::sync::Mutex;

        let surface = Surface::new(camera.image_width as usize,
                                   camera.image_height as usize,
                                   ColorRGBA::new_rgb(0, 0, 0));
        let surface = Mutex::new(surface);

        let tasks = self.tasks.unwrap_or_else(num_cpus::get);
        let pool = ThreadPool::new(
            Configuration::new()
                .num_threads(tasks)).unwrap();

        pool.scope(|s| {
            for subsurface_factory in surface.lock().unwrap().divide(128, 8) {
                s.spawn(|_| {
                    let subsurface = Renderer::render_tile(camera, scene,
                        self.options, subsurface_factory);
                    surface.lock().unwrap().merge(&subsurface);
                });
            }
        });
    
        surface.into_inner().unwrap()
    }

    fn render_tile(camera: &Camera, scene: &Scene, options: RenderOptions, tile_factory: SurfaceFactory) -> Surface {
        let mut tile = tile_factory.create();
        let mut rng: Isaac64Rng = thread_rng().gen();
        let pixel_samples = options.pixel_samples;

        for rel_y in 0usize..tile.height {
            let abs_y = camera.image_height as usize - (tile.y_off + rel_y) - 1;
            for rel_x in 0usize..tile.width {
                let abs_x = tile.x_off + rel_x;

                // Supersampling, jitter algorithm
                let pixel_width = 1.0 / f64::from(pixel_samples);
                let mut color = ColorRGBA::black();

                for y_subpixel in 0u32..pixel_samples {
                    for x_subpixel in 0u32..pixel_samples {
                        // Don't jitter if not antialiasing
                        let (j_x, j_y) = if pixel_samples > 1 {
                            (f64::from(x_subpixel) * pixel_width + rng.gen::<f64>() * pixel_width,
                             f64::from(y_subpixel) * pixel_width + rng.gen::<f64>() * pixel_width)
                        } else {
                            (0.0, 0.0)
                        };

                        let ray = camera.get_ray(abs_x as f64 + j_x, abs_y as f64 + j_y);
                        let result = Renderer::trace(scene, &ray, options, false);
                        // Clamp subpixels for now to avoid intense aliasing when combined value is clamped later
                        // Should think of a better way to handle this
                        color = color + (result.clamp() * (1.0 / f64::from(pixel_samples * pixel_samples)));
                    }
                }
                tile[(rel_x, rel_y)] = color.into_rgba8888_noalpha();
            }
        }

        tile
    }

    fn trace(scene: &Scene, ray: &Ray, options: RenderOptions, inside: bool) -> ColorRGBA<f64> {
        if options.reflect_depth == 0 || options.refract_depth == 0 {
            return ColorRGBA::black();
        }

        match ray.get_nearest_hit(scene) {
            Some(hit) => {
                let n = hit.n.unit();
                let i = (-ray.direction).unit();

                // Local lighting computation: surface shading, shadows

                let mut result = scene.lights.iter().fold(ColorRGBA::black(), |color_acc, light| {
                    let shadow = Renderer::shadow_intensity(scene, &hit, light, options.shadow_samples);
                    let l = (light.center() - hit.position).unit();

                    color_acc + light.color() * hit.material.sample(n, i, l, hit.u, hit.v) * shadow
                });

                // Global lighting computation: reflections, refractions
                if hit.material.is_reflective() || hit.material.is_refractive() {
                    let reflect_fresnel = Renderer::fresnel_reflect(hit.material.ior(), &i, &n, inside);
                    let refract_fresnel = 1.0 - reflect_fresnel;

                    if hit.material.is_reflective() {
                        result = result + Renderer::global_reflection(scene, &hit, options, inside,
                                                                      &i, &n, reflect_fresnel);
                    }

                    if hit.material.is_refractive() {
                        result = result + Renderer::global_transmission(scene, &hit, options, inside,
                                                                        &i, &n, refract_fresnel);
                    }
                }

                result
            },
            None => scene.background.color(ray.direction)
        }
    }

    fn global_reflection(scene: &Scene, hit: &Intersection, options: RenderOptions, inside: bool,
                         i: &Vec3, n: &Vec3, reflect_fresnel: f64) -> ColorRGBA<f64> {

        let r = Vec3::reflect(i, n);
        let reflect_ray = Ray::new(hit.position, r);
        let next_reflect_options = RenderOptions { reflect_depth: options.reflect_depth - 1, ..options };

        let reflection = if hit.material.is_glossy() {
            // For glossy materials, average multiple perturbed reflection rays
            // Potential overflow by scaling after everything is done instead of scaling every iteration?
            (0..options.gloss_samples).fold(ColorRGBA::black(), |acc, _| {
                let gloss_reflect_ray = reflect_ray.perturb(hit.material.glossiness());
                acc + Renderer::trace(scene, &gloss_reflect_ray, next_reflect_options, inside)
            }) * (1.0 / f64::from(options.gloss_samples))
        } else {
            // For mirror-like materials just shoot a perfectly reflected ray instead
            Renderer::trace(scene, &reflect_ray, next_reflect_options, inside)
        };

        hit.material.global_specular(&reflection) * reflect_fresnel
    }

    fn global_transmission(scene: &Scene, hit: &Intersection, options: RenderOptions, inside: bool,
                           i: &Vec3, n: &Vec3, refract_fresnel: f64) -> ColorRGBA<f64> {

        let (t, actual_refract_fresnel) = match Vec3::refract(i, n, hit.material.ior(), inside) {
            Some(ref t) => (*t, refract_fresnel),
            None => {
                (Vec3::reflect(i, n), 1.0) // Fresnel of 1.0 = total internal reflection (TODO: verify)
            }
        };

        // Offset ray origin by EPSILON * direction to avoid hitting self when refracting
        let refract_ray = Ray::new(hit.position + t.scale(EPSILON), t);
        let next_refract_options = RenderOptions { refract_depth: options.refract_depth - 1, ..options };
        let refraction = Renderer::trace(scene, &refract_ray, next_refract_options, !inside);

        hit.material.global_transmissive(&refraction) * actual_refract_fresnel
    }

    fn shadow_intensity(scene: &Scene, hit: &Intersection,
                        light: &Light, shadow_samples: u32) -> ColorRGBA<f64> {

        if shadow_samples == 0 {
            return ColorRGBA::white();
        }

        // Point light speedup (no point in sampling a point light multiple times)
        let shadow_sample_tries = if light.is_point() { 1 } else { shadow_samples };
        let mut shadow = ColorRGBA::black();

        // Take average shadow color after jittering/sampling light position
        for _ in 0..shadow_sample_tries {
            // L has to be a unit vector for t_max 1:1 correspondence to
            // distance to light to work. Shadow feelers only search up
            // until light source.
            let sampled_light_position = light.position();
            let shadow_l = (sampled_light_position - hit.position).unit();
            let shadow_ray = Ray::new(hit.position, shadow_l);
            let distance_to_light = (sampled_light_position - hit.position).len();

            // Check against candidate primitives in scene for occlusion
            // and multiply shadow color by occluders' shadow colors
            let candidate_nodes = scene.octree.intersect_iter(&shadow_ray);

            shadow = shadow + candidate_nodes.fold(ColorRGBA::white(), |shadow_acc, prim| {
                if let Some(occ) = prim.intersects(&shadow_ray, EPSILON, distance_to_light) {
                    shadow_acc * occ.material.transmission()
                } else {
                    shadow_acc
                }
            });
        }

        shadow * (1.0 / f64::from(shadow_sample_tries))
    }

    /// Calculates the fresnel (reflectivity) given the index of refraction and the cos_angle
    /// This uses Schlick's approximation. cos_angle is normal_dot_incoming
    /// http://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    fn fresnel_reflect(ior: f64, i: &Vec3, n: &Vec3, inside: bool) -> f64 {
        let (n1, n2) = if inside { (ior, 1.0) } else { (1.0, ior) };
        let actual_n = if inside { -*n } else { *n };

        let r0_sqrt = (n1 - n2) / (n1 + n2);
        let r0 = r0_sqrt * r0_sqrt;

        let cos_angle = if n1 <= n2 {
            i.dot(&actual_n)
        } else {
            let t = match Vec3::refract(i, &-actual_n, ior, inside) {
                Some(x) => x,
                None => return 1.0 // n1 > n2 && TIR
            };

            -actual_n.dot(&t) // n1 > n2 && !TIR
        };

        let cos_term = 1.0 - cos_angle;

        (r0 + ((1.0 - r0) * cos_term * cos_term * cos_term * cos_term * cos_term)).max(0.0).min(1.0)
    }
}

#[test]
fn it_renders_the_background_of_an_empty_scene() {
    let camera = Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        45.0,
        32,
        32
    );

    let test_scene = Scene {
        lights: vec!(),
        octree: vec!().into_iter().collect(),
        background: ::ColorBackground::black(),
    };

    let shared_scene = Arc::new(test_scene);

    let render_options = RenderOptions {
        reflect_depth: 1,
        refract_depth: 1,
        shadow_samples: 1,
        gloss_samples: 1,
        pixel_samples: 1,
    };


    let renderer = Renderer {
        options: render_options,
        tasks: 2,
    };

    let image_data = renderer.render(camera, shared_scene);

    for color in image_data.buffer.iter() {
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
}
