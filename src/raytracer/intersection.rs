use material::Material;
use vec3::Vec3;

pub struct Intersection<'a> {
    pub n: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub position: Vec3,
    pub material: &'a (Material + 'a)
}
