mod bbox;
mod mesh;
mod prim;

pub use self::mesh::Mesh;
pub use self::bbox::BBox;
pub use self::prim::Prim;
pub use self::prim::plane::Plane;
pub use self::prim::sphere::Sphere;
pub use self::prim::triangle::{Triangle, TriangleOptions};


use ::material::Material;
use ::util::Vec3;

pub trait PartialBoundingBox {
    fn partial_bounding_box(&self) -> Option<BBox>;
}

pub struct Intersection<'a> {
    pub n: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub position: Vec3,
    pub material: &'a Material
}
