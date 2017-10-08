use light::Light;
use material::CubeMap;
use geometry::Prim;
use raytracer::Octree;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<Light>>,
    pub octree: Octree<Box<Prim>>,
    pub background: Vec3,
    pub skybox: Option<CubeMap>
}
