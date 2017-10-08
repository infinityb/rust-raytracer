use ::compositor::ColorRGBA;
use rand::{thread_rng, Rng};
use super::Light;
use ::util::Vec3;

#[allow(dead_code)]
pub struct SphereLight {
    pub position: Vec3,
    pub color: ColorRGBA<f64>,
    pub radius: f64
}

impl Light for SphereLight {
    /// XXX(sell): make more ECS-like?
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
