use std::slice::Iter;
use std::iter::FromIterator;
use geometry::{BBox, PartialBoundingBox};
use raytracer::Ray;
use vec3::Vec3;

//

pub struct Octree<T> where T: PartialBoundingBox {
    prims: Vec<T>,
    infinites: Vec<T>, // for infinite prims (planes)
    root: OctreeNode,
}

impl<T> FromIterator<T> for Octree<T> where T: PartialBoundingBox {
    fn from_iter<I>(iterator: I) -> Self where I: IntoIterator<Item=T> {
        let iterator = iterator.into_iter();

        let (finites, infinites): (Vec<T>, Vec<T>) =
            iterator.partition(|item| item.partial_bounding_box().is_some());

        // TODO(sell): why do we need to map here? &T isn't PartialBoundingBox,
        //             but we need to find out how to make it so.
        let bounds = BBox::from_union(finites.iter().map(|i| i.partial_bounding_box()))
            .unwrap_or(BBox::zero());

        // pbrt recommended max depth for a k-d tree (though, we're using an octree)
        // For a k-d tree: 8 + 1.3 * log2(N)
        let depth = (1.2 * (finites.len() as f64).log(8.0)).round() as i32;

        println!("Octree maximum depth {}", depth);
        let mut root_node = OctreeNode::new(bounds, depth);
        for (i, prim) in finites.iter().enumerate() {
            root_node.insert(i, prim.partial_bounding_box().unwrap());
        }

        Octree {
            prims: finites,
            infinites: infinites,
            root: root_node,
        }
    }
}

impl<T> Octree<T> where T: PartialBoundingBox {
    pub fn intersect_iter<'a>(&'a self, ray: &'a Ray) -> OctreeIterator<'a, T> {
        OctreeIterator::new(self, ray)
    }
}

pub struct OctreeNode {
    bbox: BBox,
    depth: i32,
    children: Vec<OctreeNode>,
    leaf_data: Vec<OctreeData>,
}

#[derive(Clone, Copy)]
struct OctreeData {
    pub bbox: BBox,
    pub index: usize
}

impl OctreeNode {
    #[allow(dead_code)]
    pub fn new(bbox: BBox, depth: i32) -> OctreeNode {
        OctreeNode {
            bbox: bbox,
            depth: depth,
            children: Vec::new(),
            leaf_data: Vec::new(),
        }
    }

    fn subdivide(&mut self) {
        for x in 0u32..2 {
            for y in 0u32..2 {
                for z in 0u32..2 {
                    let len = self.bbox.len();

                    let child_bbox = BBox {
                        min: Vec3 {
                            x: self.bbox.min.x + x as f64 * len.x / 2.0,
                            y: self.bbox.min.y + y as f64 * len.y / 2.0,
                            z: self.bbox.min.z + z as f64 * len.z / 2.0
                        },
                        max: Vec3 {
                            x: self.bbox.max.x - (1 - x) as f64 * len.x / 2.0,
                            y: self.bbox.max.y - (1 - y) as f64 * len.y / 2.0,
                            z: self.bbox.max.z - (1 - z) as f64 * len.z / 2.0,
                        }
                    };

                    self.children.push(OctreeNode::new(child_bbox, self.depth - 1));
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, index: usize, object_bbox: BBox) -> () {
        // Max depth
        if self.depth <= 0 {
            self.leaf_data.push(OctreeData { index: index, bbox: object_bbox });
            return;
        }

        // Empty leaf node
        if self.is_leaf() && self.leaf_data.len() == 0 {
            self.leaf_data.push(OctreeData { index: index, bbox: object_bbox });
            return;
        }

        // Occupied leaf node and not max depth: subdivide node
        if self.is_leaf() && self.leaf_data.len() == 1 {
            self.subdivide();
            let old = self.leaf_data.remove(0);
            // Reinsert old node and then fall through to insert current object
            self.insert(old.index, old.bbox);
        }

        // Interior node (has children)
        for child in self.children.iter_mut() {
            if child.bbox.overlaps(&object_bbox) {
                child.insert(index, object_bbox);
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

pub struct OctreeIterator<'a, T:'a> {
    prims: &'a [T],
    stack: Vec<&'a OctreeNode>,
    leaf_iter: Option<Iter<'a, OctreeData>>,
    ray: &'a Ray,
    infinites: Iter<'a, T>,
    just_infinites: bool
}


impl<'a, T> OctreeIterator<'a, T> where T: PartialBoundingBox {
    fn new<'b>(octree: &'b Octree<T>, ray: &'b Ray) -> OctreeIterator<'b, T> {
        OctreeIterator {
            prims: &octree.prims[..],
            stack: vec![&octree.root],
            leaf_iter: None,
            ray: ray,
            infinites: octree.infinites.iter(),
            just_infinites: false
        }
    }
}


impl<'a, T> Iterator for OctreeIterator<'a, T> where T: PartialBoundingBox {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.just_infinites {
            return self.infinites.next();
        }

        loop {
            let ray = self.ray;
            if let Some(leaf_iter) = self.leaf_iter.as_mut() {
                if let Some(val) = leaf_iter.filter(|x| x.bbox.intersects(ray)).next() {
                    return Some(&self.prims[val.index]);
                }
                // iterator went empty, so we'll pop from the stack and
                // iterate on the next node's children now,
            }

            if let Some(node) = self.stack.pop() {
                for child in node.children.iter() {
                    if child.bbox.intersects(self.ray) {
                        self.stack.push(child);
                    }
                }
                self.leaf_iter = Some(node.leaf_data.iter());
            } else {
                self.just_infinites = true;
                return self.infinites.next()
            }
        }
    }
}
