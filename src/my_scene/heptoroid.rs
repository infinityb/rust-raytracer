use raytracer::compositor::ColorRGBA;
use raytracer::geometry::Prim;
use raytracer::light::{Light, SphereLight};
use raytracer::material::CookTorranceMaterial;
use ::textures::CubeMap;
use raytracer::scene::{Camera, Scene};
use raytracer::util::Vec3;

// 114688 tris, 57302 verts
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 7.0, y: 2.0, z: -6.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene(material_option: HeptoroidMaterial) -> Scene {
    let mut lights: Vec<Box<Light>> = Vec::new();
    lights.push(Box::new(SphereLight {
        position: Vec3 { x: 2.0, y: 3.0, z: -2.0 },
        color: ColorRGBA::new_rgb(1.0, 1.0, 1.0),
        radius: 1.0
    }));

    let heptoroid_material = match material_option {
        HeptoroidMaterial::Shiny => {
            CookTorranceMaterial {
                k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 0.55, k_tg: 0.0,
                gauss_constant: 5.0, roughness: 0.01, glossiness: 0.0, ior: 0.25,
                ambient: ColorRGBA::white(),
                diffuse: ColorRGBA::white(),
                specular: ColorRGBA::white() * 0.9,
                transmission: ColorRGBA::black(),
                diffuse_texture: None
            }
        }
        HeptoroidMaterial::Refractive => {
            CookTorranceMaterial {
                k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0,
                gauss_constant: 5.0, roughness: 0.01, glossiness: 0.0, ior: 1.50,
                ambient: ColorRGBA::white(),
                diffuse: ColorRGBA::white(),
                specular: ColorRGBA::white() * 0.9,
                transmission: ColorRGBA::white() * 0.8,
                diffuse_texture: None
            }
        }
        HeptoroidMaterial::White => {
            CookTorranceMaterial {
                k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 0.15, k_tg: 0.0,
                gauss_constant: 5.0, roughness: 0.1, ior: 0.5, glossiness: 0.0,
                ambient: ColorRGBA::white(),
                diffuse: ColorRGBA::new_rgb(0.9, 0.85, 0.7),
                specular: ColorRGBA::white(),
                transmission: ColorRGBA::black(),
                diffuse_texture: None }
        }
    };

    let mut prims: Vec<Box<Prim>> = Vec::new();
    let heptoroid = ::util::import::from_obj(Box::new(heptoroid_material), false, "./docs/assets/models/heptoroid.obj")
        .expect("failed to load obj model");

    for triangle in heptoroid.triangles {
        prims.push(triangle);
    }

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        // background: Vec3 { x: 0.84, y: 0.34, z: 0.0 },
        background: Box::new(CubeMap::load(
            "./docs/assets/textures/skyboxes/miramar_y_up/left.png",
            "./docs/assets/textures/skyboxes/miramar_y_up/right.png",
            "./docs/assets/textures/skyboxes/miramar_y_up/down.png",
            "./docs/assets/textures/skyboxes/miramar_y_up/up.png",
            "./docs/assets/textures/skyboxes/miramar_y_up/front.png",
            "./docs/assets/textures/skyboxes/miramar_y_up/back.png"
        ))
    }
}

#[derive(Copy, Clone)]
pub enum HeptoroidMaterial {
    Shiny,
    Refractive,
    White,
}

pub struct HeptoroidConfig {
    material: HeptoroidMaterial,
}

impl HeptoroidConfig {
    pub fn shiny() -> HeptoroidConfig {
        HeptoroidConfig { material: HeptoroidMaterial::Shiny }
    }

    pub fn white() -> HeptoroidConfig {
        HeptoroidConfig { material: HeptoroidMaterial::White }
    }

    pub fn refractive() -> HeptoroidConfig {
        HeptoroidConfig { material: HeptoroidMaterial::Refractive }
    }
}

impl super::SceneConfig for HeptoroidConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        get_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene {
        get_scene(self.material)
    }
}