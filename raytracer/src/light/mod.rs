mod pointlight;
mod spherelight;

use ::compositor::ColorRGBA;
pub use self::pointlight::PointLight;
pub use self::spherelight::SphereLight;

use ::util::Vec3;


pub trait Light: Send + Sync {
    fn position(&self) -> Vec3;

    fn color(&self) -> ColorRGBA<f64>;

    fn center(&self) -> Vec3;

    fn is_point(&self) -> bool;
}

impl Light for Box<Light> {
    fn position(&self) -> Vec3 {
    	(**self).position()
    }

    fn color(&self) -> ColorRGBA<f64> {
    	(**self).color()
    }

    fn center(&self) -> Vec3 {
    	(**self).center()
    }

    fn is_point(&self) -> bool {
    	(**self).is_point()
    }
}
