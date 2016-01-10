use geometry::{BBox, PartialBoundingBox};
use raytracer::{Ray, Intersection};
use mat4::Transform;
use rtree::{BBox as RTreeBBox, Vec3 as RTreeVec3, Mbr};

pub trait Prim: PartialBoundingBox {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>>;

    // fn transform(&self, transform: &Transform) -> Box<Prim+Send+Sync>;
    fn mut_transform(&mut self, transform: &Transform);
}

impl<'a> PartialBoundingBox for Box<Prim+Send+Sync> {
    fn partial_bounding_box(&self) -> Option<BBox> {
        (**self).partial_bounding_box()
    }
}

impl<'a> Mbr for Box<Prim+Send+Sync> {
    fn mbr(&self) -> Option<RTreeBBox> {
        self.partial_bounding_box().map(|bbox| RTreeBBox {
            min: RTreeVec3 { x: bbox.min.x, y: bbox.min.y, z: bbox.min.z },
            max: RTreeVec3 { x: bbox.max.x, y: bbox.max.y, z: bbox.max.z },
        })
    }
}
