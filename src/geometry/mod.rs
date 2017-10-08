mod bbox;
mod mesh;
mod prim;

pub use self::mesh::Mesh;
pub use self::bbox::BBox;
pub use self::prim::Prim;
pub use self::prim::plane::Plane;
pub use self::prim::sphere::Sphere;
pub use self::prim::triangle::{Triangle, TriangleOptions};

pub trait PartialBoundingBox {
    fn partial_bounding_box(&self) -> Option<BBox>;
}
