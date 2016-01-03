#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::animator::CameraKeyframe;
use raytracer::animator::easing::Easing;
use raytracer::compositor::ColorRGBA;
use scene::{Camera, Scene};
use vec3::Vec3;

// ~70K triangles, no textures yet
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: -16.0, y: -14.5, z: -2.0 },
        Vec3 { x: 8.0, y: -3.0, z: 2.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}
// 7s target length
pub fn get_animation_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new_with_keyframes(
        Vec3 { x: -16.0, y: -14.5, z: -2.0 },
        Vec3 { x: 8.0, y: -3.0, z: 2.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 7.0,
                position: Vec3 { x: 8.0, y: -13.5, z: 0.2 },
                look_at: Vec3 { x: 8.5, y: 8.0, z: 2.0 },
                up: Vec3 { x: -0.9, y: 0.0, z: -0.7 },
                easing: Easing::linear()
            }
        ]
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight { position: Vec3 { x: 8.0, y: 8.0, z: 0.0 }, color: Vec3 { x: 1.0, y: 0.8, z: 0.4}, radius: 0.5 }));
    lights.push(Box::new(SphereLight { position: Vec3 { x: 8.0, y: -5.0, z: 0.0 }, color: Vec3 { x: 0.5, y: 0.4, z: 0.2}, radius: 1.0 }));
    lights.push(Box::new(PointLight { position: Vec3 { x: -16.0, y: -14.5, z: -2.0 }, color: Vec3 { x: 0.15, y: 0.07, z: 0.05 } }));


    let checker: Box<Texture+Send+Sync> = Box::new(CheckerTexture { color1: ColorRGBA::white(), color2: ColorRGBA::new_rgb(0.15, 0.11, 0.1), scale: 1.0 });

    let stone     = CookTorranceMaterial { k_a: 0.1,  k_d: 0.8, k_s: 0.2, k_sg: 0.0, k_tg: 0.0, gauss_constant: 25.0, roughness: 1.0, glossiness: 0.0, ior: 1.5, ambient: Vec3 { x: 0.88, y: 0.83, z: 0.77 }, diffuse: Vec3 { x: 0.88, y: 0.83, z: 0.77 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let ground    = CookTorranceMaterial { k_a: 0.03, k_d: 0.9, k_s: 0.3, k_sg: 0.5, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.1, glossiness: 0.0, ior: 0.5, ambient: Vec3::one(), diffuse: Vec3 { x: 0.38, y: 0.38, z: 0.5 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone()) };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    prims.push(Box::new(Plane { a: 0.0,  b: -1.0, c: 0.0, d: -14.9, material: Box::new(ground.clone()) }));

    let sibenik = ::util::import::from_obj(stone, false, "./docs/assets/models/sibenik.obj").ok().expect("failed to load obj model");;
    for triangle in sibenik.triangles.into_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
        skybox: None
    }
}
