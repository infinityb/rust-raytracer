use super::Light;
use vec3::Vec3;

#[allow(dead_code)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3
}

impl Light for PointLight {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn color(&self) -> Vec3 {
        self.color
    }

    fn center(&self) -> Vec3 {
        self.position
    }

    fn is_point(&self) -> bool {
        true
    }
}
