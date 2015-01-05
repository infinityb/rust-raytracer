use std::rand::{thread_rng, Rng};
use light::light::Light;
use raytracer::compositor::ColorRGBA;
use vec3::Vec3;

#[allow(dead_code)]
pub struct SphereLight {
    pub position: Vec3,
    pub color: ColorRGBA<f64>,
    pub radius: f64
}

impl Light for SphereLight {
    fn position(&self) -> Vec3 {
        let mut rng = thread_rng();

        let jitter = Vec3 {
            x: self.radius * (rng.gen::<f64>() - 0.5),
            y: self.radius * (rng.gen::<f64>() - 0.5),
            z: self.radius * (rng.gen::<f64>() - 0.5)
        };

        self.position + jitter
    }

    fn color(&self) -> ColorRGBA<f64> {
        self.color
    }

    fn center(&self) -> Vec3 {
        self.position
    }

    fn is_point(&self) -> bool {
        false
    }
}
