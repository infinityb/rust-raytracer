use std::cmp::{min, max};

use super::{BBox, Mbr};
use super::vec3::Vec3;
use super::ray::Ray;

const SPHERE_RADIUS_TOO_SMALL: &'static str = "sphere radius must be above zero";

#[derive(Debug)]
pub struct Sphere {
    origin: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64) -> Result<Sphere, &'static str> {
        if radius <= 0.0 {
            return Err(SPHERE_RADIUS_TOO_SMALL);
        }
        Ok(Sphere {
            origin: origin,
            radius: radius,
        })
    }
}

impl Mbr for Sphere {
    fn mbr(&self) -> BBox {
        BBox {
            min: self.origin - self.radius,
            max: self.origin + self.radius,
        }
    }
}
