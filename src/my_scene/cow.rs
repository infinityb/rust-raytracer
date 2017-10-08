#![allow(unused_imports)]

use raytracer::ColorBackground;
use raytracer::compositor::{ColorRGBA};
use raytracer::geometry::{Prim, Plane, Sphere, Triangle, TriangleOptions};
use raytracer::light::{Light, PointLight, SphereLight};
use raytracer::material::{Material, CookTorranceMaterial, FlatMaterial, PhongMaterial};
use raytracer::material::{Texture, CheckerTexture, UVTexture};
use ::textures::{CubeMap, ImageTexture};
// use raytracer::animator::CameraKeyframe;
use raytracer::scene::{Camera, Scene};
use raytracer::util::Vec3;

// 5000 polys, cow. Octree helps.
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: -2.0, y: 4.0, z: 10.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light>> = Vec::new();
    lights.push(Box::new(SphereLight {
        position: Vec3 {x: 3.0, y: 10.0, z: 6.0},
        color: ColorRGBA::white(),
        radius: 5.0,
    }));

    let red = CookTorranceMaterial {
        k_a: 0.0, k_d: 0.6, k_s: 1.0, k_sg: 0.2, k_tg: 0.0,
        gauss_constant: 30.0, roughness: 0.1, glossiness: 0.0, ior: 0.8,
        ambient: ColorRGBA::white(),
        diffuse: ColorRGBA::new_rgb(1.0, 0.25, 0.1),
        specular: ColorRGBA::white(),
        transmission: ColorRGBA::black(),
        diffuse_texture: None
    };
    let green = CookTorranceMaterial {
        k_a: 0.0, k_d: 0.5, k_s: 0.4, k_sg: 0.1, k_tg: 0.0,
        gauss_constant: 25.0, roughness: 0.4, glossiness: 0.0, ior: 0.95,
        ambient: ColorRGBA::white(),
        diffuse: ColorRGBA::new_rgb(0.2, 0.7, 0.2),
        specular: ColorRGBA::white(),
        transmission: ColorRGBA::black(),
        diffuse_texture: None
    };

    let mut prims: Vec<Box<Prim>> = Vec::new();
    prims.push(Box::new(Plane { a: 0.0, b: 1.0, c: 0.0, d: 3.6, material: Box::new(green) }));
    let cow = ::util::import::from_obj(Box::new(red), true, "./docs/assets/models/cow.obj").expect("failed to load obj model");;
    for triangle in cow.triangles { prims.push(triangle); }

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Box::new(ColorBackground::new(ColorRGBA::new_rgb(0.3, 0.5, 0.8))),
    }
}

pub struct CowConfig;

impl super::SceneConfig for CowConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        get_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene {
        get_scene()
    }
}