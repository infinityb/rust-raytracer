use raytracer::compositor::ColorRGBA;
use vec3::Vec3;

pub trait Light {
    fn position(&self) -> Vec3;
    fn color(&self) -> ColorRGBA<f64>;
    fn center(&self) -> Vec3;
    fn is_point(&self) -> bool;
}
