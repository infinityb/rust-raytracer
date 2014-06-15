pub use self::prim::Prim;

pub mod prim;

pub mod Prims {
    pub use self::plane::Plane;
    pub use self::sphere::Sphere;
    pub use self::triangle::Triangle;

    mod plane;
    mod sphere;
    mod triangle;
}