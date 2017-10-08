mod pointlight;
mod spherelight;

pub use self::pointlight::PointLight;
pub use self::spherelight::SphereLight;

use vec3::Vec3;


pub trait Light: Send + Sync {
    fn position(&self) -> Vec3;

    fn color(&self) -> Vec3;

    fn center(&self) -> Vec3;

    fn is_point(&self) -> bool;
}

impl Light for Box<Light> {
    fn position(&self) -> Vec3 {
    	(**self).position()
    }

    fn color(&self) -> Vec3 {
    	(**self).color()
    }

    fn center(&self) -> Vec3 {
    	(**self).center()
    }

    fn is_point(&self) -> bool {
    	(**self).is_point()
    }
}
