pub use self::prim::Prim;
pub use self::mesh::Mesh;
pub use self::bbox::BBox;
pub use self::icosahedron::IcosahedronBuilder;

pub mod bbox;
pub mod prim;
pub mod mesh;
pub mod icosahedron;

pub mod prims {
    pub use self::plane::Plane;
    pub use self::sphere::Sphere;
    pub use self::triangle::Triangle;
    pub use self::triangle::TriangleVertex;

    mod plane;
    mod sphere;
    mod triangle;
}
