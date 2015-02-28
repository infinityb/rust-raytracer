#![allow(unused_imports)]

use geometry::IcosahedronBuilder;
use geometry::icosahedron::painters;
use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::Octree;
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// Skybox test scene
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    let up = Vec3 { x: 0.0, y: 1.0, z: 0.0 }; // y-up
    Camera::new(
        Vec3 { x: -10.0, y: 10.0, z: 10.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        up,
        fov,
        image_width,
        image_height,
    )
}

pub fn get_animation_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    // State at time t=0
    // A keyframe at time t=0 is automatically created when insert_keyframes is called
    let camera = Camera::new_with_keyframes(
        Vec3 { x: -10.0, y: 10.0, z: 10.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 2.5,
                position: Vec3 { x: 10.0, y: 10.0, z: 0.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            CameraKeyframe {
                time: 5.0,
                position: Vec3 { x: 0.0, y: 10.0, z: -10.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            CameraKeyframe {
                time: 7.5,
                position: Vec3 { x: -10.0, y: 10.0, z: 0.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            CameraKeyframe {
                time: 10.0,
                position: Vec3 { x: 0.0, y: 10.0, z: 10.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
        ]
    );

    camera
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(box SphereLight {
        position: Vec3 { x: 3000.0, y: 10000.0, z: 6000.0 },
        color: Vec3::one() * 0.65,
        radius: 500.0
    });

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let shiny = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.05, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };


    for &z in [-1.0, 0.0, 1.0].iter() {
        for &y in [-1.0, 0.0, 1.0].iter() {
            for &x in [-1.0, 0.0, 1.0].iter() {
                if x * y * z < 0.0 && 0.0 < y {
                    lights.push(box SphereLight {
                        position: Vec3 { x: 300.0 * x, y: 600.0 * y, z: 900.0 * z },
                        color: Vec3::one() / 11.0,
                        radius: 50.0
                    });
                }

                // prims.push(box Sphere {
                //     center: Vec3 { x: 2.5 * x, y: 2.5 * y, z: 2.5 * z },
                //     radius: 0.15,
                //     material: box shiny.clone()
                // });
            }
        }
    }    

    let paint = painters::TexturePainter::new(
        ImageTexture::load("./earthmap_hires.ppm").image);

    prims.extend(IcosahedronBuilder::new()
        .center(Vec3 { x: 0.0, y: 0.0, z: 0.0 })
        .radius(3.5)
        .max_depth(9)
        .paint(paint)
        .build_sphere()
        .map(|prim| Box::new(prim) as Box<Prim+Send+Sync>));

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    // For y as up
    Scene {
        lights: lights,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
        octree: octree,
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/back.ppm"
        ))
    }
}
