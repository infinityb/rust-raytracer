use light::light::Light;
use raytracer::compositor::ColorRGBA;
use vec3::Vec3;

#[allow(dead_code)]
pub struct PointLight {
    pub position: Vec3,
    pub color: ColorRGBA<f64>,
}

impl Light for PointLight {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn color(&self) -> ColorRGBA<f64> {
        self.color
    }

    fn center(&self) -> Vec3 {
        self.position
    }

    fn is_point(&self) -> bool {
        true
    }
}
