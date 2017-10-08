pub mod plane;
pub mod sphere;
pub mod triangle;

use ::geometry::Intersection;
use ::mat4::{Transform};
use ::{Ray};
use super::{BBox, PartialBoundingBox};

pub trait Prim: PartialBoundingBox + Send + Sync {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>>;
    
    fn mut_transform(&mut self, transform: &Transform);
}

impl<'a> PartialBoundingBox for Box<Prim> {
    fn partial_bounding_box(&self) -> Option<BBox> {
        (**self).partial_bounding_box()
    }
}