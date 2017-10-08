extern crate rand;
extern crate rayon;
extern crate num;
extern crate num_cpus;


pub mod octree;
pub mod ray;
mod renderer;
pub mod util;
pub mod material;
pub mod geometry;
pub mod compositor;
pub mod scene;
pub mod mat4;
pub mod light;


pub use self::ray::Ray;
pub use self::scene::Scene;
pub use self::renderer::{Renderer, RenderOptions};

use ::util::Vec3;
use ::compositor::ColorRGBA;

pub trait Background: Send + Sync {
    fn color(&self, dir: Vec3) -> ColorRGBA<f64>;
}

pub struct ColorBackground {
    color: ColorRGBA<f64>,
}

impl ColorBackground {
    pub fn new(color: ColorRGBA<f64>) -> ColorBackground {
        ColorBackground { color }
    }

    pub fn black() -> ColorBackground {
        ColorBackground {
            color: ColorRGBA::black(),
        }
    }
}

impl Background for ColorBackground {
    fn color(&self, _: Vec3) -> ColorRGBA<f64> {
        self.color
    }
}