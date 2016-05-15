#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::{Material, Texture};
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::animator::CameraKeyframe;
use raytracer::animator::easing::Easing;
use scene::{Camera, AnimatedCamera, Scene};
use vec3::Vec3;
use rand::{Rand, Rng, Isaac64Rng, thread_rng};

// Skybox test scene
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    let up = Vec3::xyz(0., 0., 1.,); // z-up
    Camera::new(
        Vec3::xyz(0., 45., 0.),
        Vec3::xyz(0., 0., 0.),
        up,
        fov,
        image_width,
        image_height
    )
}

#[derive(Clone, Copy)]
enum MaterialFactory {
    Porcelain,
    Shiny,
}

impl Rand for MaterialFactory {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let options = &[
            MaterialFactory::Porcelain,
            MaterialFactory::Shiny,
        ];
        *rng.choose(options).unwrap()
    }
}

impl MaterialFactory {
    fn create(&self, rng: &mut Isaac64Rng) -> Box<Material+Send+Sync> {
        let create_fn = self.as_fn();
        create_fn(rng)
    }

    fn as_fn(&self) -> fn(&mut Isaac64Rng) -> Box<Material+Send+Sync> {
        match *self {
            MaterialFactory::Porcelain => get_porcelain,
            MaterialFactory::Shiny => get_shiny,
        }
    }
}

fn get_porcelain(_rng: &mut Isaac64Rng) -> Box<Material+Send+Sync> {
    Box::new(CookTorranceMaterial {
        k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 1.0, k_tg: 0.0,
        gauss_constant: 5.0,
        roughness: 0.1,
        glossiness: 0.0,
        ior: 1.1,
        ambient: Vec3::one(),
        diffuse: Vec3::xyz(0.9, 0.85, 0.7),
        specular: Vec3::one(),
        transmission: Vec3::zero(),
        diffuse_texture: None
    })
}

fn get_shiny(rng: &mut Isaac64Rng) -> Box<Material+Send+Sync> {
    Box::new(CookTorranceMaterial {
        k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0,
        gauss_constant: 5.0,
        roughness: 0.20,
        glossiness: 0.0,
        ior: 0.05,
        ambient: rng.gen::<Vec3>().unit(),
        diffuse: Vec3::zero(),
        specular: Vec3::zero(),
        transmission: Vec3::zero(),
        diffuse_texture: None
    })
}

pub fn get_scene(rng: &mut Isaac64Rng) -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight {
        position: Vec3::xyz(30., 100., 60.),
        color: Vec3::one() * 5.0,
        radius: 5.,
    }));


    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();

    for _ in 0..250000 {
        let x = rng.gen_range(-200.0, 600.0);
        let y = rng.gen_range(-200.0, 600.0);
        let z = rng.gen_range(-70.0, 600.0);

        prims.push(Box::new(Sphere {
            center: Vec3::xyz(x, y, z),
            radius: 0.5,
            material: rng.gen::<MaterialFactory>().create(rng),
        }));
    }

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    // For y as up
    Scene {
        lights: lights,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
        octree: octree,
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.png",
            "./docs/assets/textures/skyboxes/storm_y_up/right.png",
            "./docs/assets/textures/skyboxes/storm_y_up/down.png",
            "./docs/assets/textures/skyboxes/storm_y_up/up.png",
            "./docs/assets/textures/skyboxes/storm_y_up/front.png",
            "./docs/assets/textures/skyboxes/storm_y_up/back.png"
        ))
    }
}


pub struct SphereConfig {
    rng: Isaac64Rng,
}

impl SphereConfig {
    pub fn new() -> SphereConfig {
        SphereConfig {
            rng: thread_rng().gen(),
        }
    }
}

impl super::SceneConfig for SphereConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        get_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene {
        // deterministic~

        let mut rng = self.rng.clone();
        get_scene(&mut rng)
    }
}