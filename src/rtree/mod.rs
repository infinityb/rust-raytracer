mod bbox;
mod vec3;
mod ray;

use std::iter::FromIterator;
use std::slice::Iter as SliceIter;
pub use self::ray::Ray;
pub use self::bbox::BBox;
pub use self::vec3::Vec3;

#[cfg(test)]
mod test_helpers;

// References:
//  * http://www.cs.sfu.ca/CourseCentral/454/jpei/slides/R-Tree.pdf
//  * http://www-db.deis.unibo.it/courses/SI-LS/papers/Gut84.pdf
//  * http://avid.cs.umass.edu/courses/645/s2007/lectures/CS645-Lec9-RTree.pdf

// number of children in any given node.
const NODE_SIZE: usize = 16;

// nodes can have as few as NODE_MIN_FILL children.
const NODE_MIN_FILL: usize = 19;

pub trait Mbr: Sized {
    fn mbr(&self) -> Option<BBox>;
}

#[must_use]
/// Represents the result of an Insertion: either the item fit, or the node
/// had to split
enum InsertionResult<T> where T: Mbr {
    /// The inserted element fit, boolean indicates whether bounding box was
    /// changed.
    Fit(bool),

    /// The inserted element did not fit, so the node was split.  Re-insert
    /// the following items.  The MBR has not changed.
    SplitMe(T),

    /// We've split a node and now we have an subtree that needs to be
    /// reinserted into the tree.  Assume the MBR has changed and recompute.
    Bubble(NodeStorage<T>),
}

impl<T> InsertionResult<T> where T: Mbr {
    fn mbr_changed(&self) -> bool {
        match *self {
            InsertionResult::Fit(val) => val,
            InsertionResult::SplitMe(_) => false,
            InsertionResult::Bubble(_) => true,
        }
    }
}

struct BBoxItem<T> {
    bbox: BBox,
    item: T,
}

impl<T> BBoxItem<T> where T: Mbr {
    fn new(item: T) -> BBoxItem<T> {
        let bbox = item.mbr().unwrap();
        BBoxItem {
            bbox: bbox,
            item: item,
        }
    }
}

impl<T> Mbr for BBoxItem<T> {
    fn mbr(&self) -> Option<BBox> {
        Some(self.bbox)
    }
}

enum NodeStorage<T> where T: Mbr {
    Interior(Vec<RTreeNode<T>>),
    Leaf(Vec<BBoxItem<T>>),
}

impl<T> NodeStorage<T> where T: Mbr {
    #[inline]
    fn new_leaf_node(item: T) -> NodeStorage<T> {
        NodeStorage::Leaf(vec![BBoxItem::new(item)])
    }

    #[inline]
    fn shallow_len(&self) -> usize {
        match *self {
            NodeStorage::Interior(ref vec) => vec.len(),
            NodeStorage::Leaf(ref vec) => vec.len(),
        }
    }

    #[inline]
    fn deep_len(&self) -> usize {
        match *self {
            NodeStorage::Interior(ref vec) => {
                vec.iter().map(|v| v.deep_len()).fold(0, |a,x| a+x)
            },
            NodeStorage::Leaf(ref vec) => vec.len(),
        }
    }
}

struct RTreeNode<T> where T: Mbr {
    bbox: BBox,
    storage: NodeStorage<T>,
}

impl<T> Mbr for RTreeNode<T> where T: Mbr {
    fn mbr(&self) -> Option<BBox> {
        Some(self.bbox)
    }
}

#[cfg(debug_assertions)]
struct Height(u32, u32);

#[cfg(debug_assertions)]
impl Height {
    fn checked(&self) -> u32 {
        assert_eq!(self.0, self.1);
        self.0
    }

    fn merge<I>(iterator: I) ->Option<Height>
        where
            I: Iterator<Item=Height>
    {
        use std::cmp::{min, max};
        let mut min_val = u32::max_value();
        let mut max_val = u32::min_value();

        for Height(new_min, new_max) in iterator {
            assert!(new_min <= new_max);
            min_val = ::std::cmp::min(new_min, min_val);
            max_val = ::std::cmp::max(new_max, max_val);
        }

        // no values in iterator
        if max_val < min_val {
            return None
        }

        Some(Height(min_val, max_val))
    }
}

#[cfg(debug_assertions)]
impl<T> RTreeNode<T> where T: Mbr {
    #[inline]
    fn height(&self) -> Height {
        match self.storage {
            NodeStorage::Interior(ref children) => {
                Height::merge(children.iter().map(|c| c.height()))
                    .expect("interior node may not be empty")
            }
            NodeStorage::Leaf(_) => Height(1, 1),
        }
    }

    #[inline]
    fn check_insert_node(&self, node: &RTreeNode<T>) {
        self.invariant();
        node.invariant();

        let self_height = self.height().checked();
        let inserting_height = node.height().checked();
        assert_eq!(self_height, inserting_height + 1);
    }

    fn invariant(&self) {
        assert!(self.shallow_len() > 0,
            "A node must have at least one element");

        match self.storage {
            NodeStorage::Interior(ref children) => {
                // assert!(children.len() > 1, "interior nodes of length one are useless.");
                for child in children.iter() {
                    child.invariant();
                }
            },
            NodeStorage::Leaf(_) => (),
        }
        let _ = self.height().checked();
    }
}

#[cfg(not(debug_assertions))]
impl<T> RTreeNode<T> where T: Mbr {
    #[inline]
    fn check_insert_node(&self, node: &RTreeNode<T>) {
        // noop
    }

    #[inline]
    fn invariant(&self) {
        // noop
    }
}

impl<T> RTreeNode<T> where T: Mbr {
    pub fn new(item: T) -> RTreeNode<T> {
        let bbox = item.mbr().unwrap();

        RTreeNode {
            bbox: bbox,
            storage: NodeStorage::new_leaf_node(item),
        }
    }

    fn new_interior(bbox: BBox, nodes: Vec<RTreeNode<T>>) -> RTreeNode<T> {
        assert!(nodes.len() > 0);
        RTreeNode {
            bbox: bbox,
            storage: NodeStorage::Interior(nodes),
        }
    }

    fn new_leaf(bbox: BBox, nodes: Vec<BBoxItem<T>>) -> RTreeNode<T> {
        assert!(nodes.len() > 0);
        RTreeNode {
            bbox: bbox,
            storage: NodeStorage::Leaf(nodes),
        }
    }

    fn is_full(&self) -> bool {
        NODE_SIZE <= self.storage.shallow_len()
    }

    fn shallow_len(&self) -> usize {
        self.storage.shallow_len()
    }

    fn deep_len(&self) -> usize {
        self.storage.deep_len()
    }

    fn split(self) -> (RTreeNode<T>, RTreeNode<T>) {
        assert_eq!(self.shallow_len(), NODE_SIZE);

        match self.storage {
            NodeStorage::Interior(children) => {
                let (lbox, lefts, rbox, rights) = impl_util::split(children);
                (
                    RTreeNode::new_interior(lbox, lefts),
                    RTreeNode::new_interior(rbox, rights),
                )
            },
            NodeStorage::Leaf(children) => {
                let (lbox, lefts, rbox, rights) = impl_util::split(children);
                (
                    RTreeNode::new_leaf(lbox, lefts),
                    RTreeNode::new_leaf(rbox, rights),
                )
            }
        }
    }

    fn insert_node(&mut self, node: RTreeNode<T>) -> InsertionResult<T> {
        self.check_insert_node(&node);
        unimplemented!();
    }

    fn insert(&mut self, item: T) -> InsertionResult<T> {
        self.invariant();

        if self.is_full() {
            return InsertionResult::SplitMe(item);
        }

        let ins_res = match self.storage {
            NodeStorage::Interior(ref mut children) => {
                interior_insert(&mut self.bbox, children, item)
            },
            NodeStorage::Leaf(ref mut children) => {
                let item_mbr = item.mbr().unwrap();
                children.push(BBoxItem::new(item));
                let mbr_mod = common_util::union_bbox(&mut self.bbox, item_mbr);
                InsertionResult::Fit(mbr_mod)
            },
        };
        self.invariant();
        ins_res
    }

    fn is_leaf(&self) -> bool {
        match self.storage {
            NodeStorage::Leaf(_) => true,
            NodeStorage::Interior(_) => false,
        }
    }
}

fn interior_insert<T>(
    bbox: &mut BBox,
    children: &mut Vec<RTreeNode<T>>,
    item: T
) -> InsertionResult<T> where T: Mbr {
    let item_bbox = item.mbr().unwrap();
    let best_child_idx = impl_util::best_fit(item_bbox, &children)
        .expect("interior nodes may not be empty");
    match children[best_child_idx].insert(item) {
        InsertionResult::SplitMe(item) => {
            // assert!(children.len() > 1, "interior nodes of length one are useless.");

            let node = children.swap_remove(best_child_idx);

            let self_bbox = common_util::mbr_from_slice(children);

            let (left, right): (RTreeNode<T>, RTreeNode<T>) = {
                let (left, right) = node.split();
                let mut both = [left, right];
                let item_bbox = item.mbr().unwrap();
                let best_idx = impl_util::best_fit(item_bbox, &both[..]).unwrap();
                both[best_idx].insert(item);
                unsafe { ::std::mem::transmute(both) }
            };

            let (left, right) = impl_util::score_swap(&self_bbox, left, right);
            let self_bbox = self_bbox.union(&left.bbox);
            children.push(left);

            let maybe_extra = match NODE_SIZE <= children.len() {
                true => Some(right),
                false => {
                    children.push(right);
                    None
                }
            };

            // recompute bounding box
            let new_bbox = common_util::mbr_from_slice(children);
            let mbr_mod = common_util::replace_bbox(bbox, new_bbox);

            match maybe_extra {
                Some(extra) => InsertionResult::Bubble(extra.storage),
                None => InsertionResult::Fit(mbr_mod)
            }
        },
        InsertionResult::Bubble(bubble) => {
            unimplemented!();
        },
        InsertionResult::Fit(true) => {
            let new_bbox = common_util::mbr_from_slice(children);
            InsertionResult::Fit(common_util::replace_bbox(bbox, new_bbox))
        },
        InsertionResult::Fit(false) => InsertionResult::Fit(false),
    }
}

pub struct RTree<T> where T: Mbr {
    root: Option<RTreeNode<T>>,
    infinites: Vec<T>,
}

impl<T> RTree<T> where T: Mbr {
    pub fn new() -> RTree<T> {
        RTree {
            root: None,
            infinites: Vec::new(),
        }
    }

    pub fn insert(&mut self, item: T) {
        if item.mbr().is_none() {
            self.infinites.push(item);
            return;
        }

        if self.root.is_none() {
            self.root = Some(RTreeNode::new(item));
            return;
        }

        let mut node = self.root.take().unwrap();
        let deep_length = node.deep_len();

        self.root = Some(match node.insert(item) {
            InsertionResult::SplitMe(item) => {
                let (left, right) = node.split();

                let bbox = left.mbr().unwrap().union(&right.mbr().unwrap());
                let nodes: Vec<RTreeNode<T>> = vec![left, right];

                RTreeNode::new_interior(bbox, nodes)
            },
            // root doesn't track MBR, so nothing to change.
            InsertionResult::Bubble(_) => unimplemented!(),
            InsertionResult::Fit(_) => node,
        });
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter::new(self)
    // }

    pub fn intersect_iter<'a>(&'a self, ray: &'a Ray) -> IntersectIter<'a, T> {
        IntersectIter::new(self, ray)
    }
}

#[cfg(debug_assertions)]
impl<T> RTree<T> where T: Mbr {
    fn assert_height(&self) {
        if self.root.is_none() {
            return;
        }

        let mut root = self.root.as_ref().unwrap();
        let Height(min, max) = root.height();
        assert_eq!(min, max);
    }
}

fn print_tree<T>(tree: &RTreeNode<T>, depth: usize) where T: Mbr {
    let indentation: String = (0..(depth * 3)).map(|_| ' ').collect();
    let node_type = match tree.is_leaf() {
        true => "leaf",
        false => "interior",
    };
    let ch_len = match tree.storage {
        NodeStorage::Leaf(ref ch) => ch.len(),
        NodeStorage::Interior(ref ch) => ch.len(),
    };
    println!("{}RTreeNode({}, bbox={:?}, {} children, ...)", indentation, node_type, tree.bbox, ch_len);
    if let NodeStorage::Interior(ref children) = tree.storage {
        for child in children.iter() {
            print_tree(child, depth + 1);
        }
    }
}

fn print_tree2<T>(tree: &RTreeNode<T>, depth: usize) where T: Mbr {
    let indentation: String = (0..(depth * 3)).map(|_| ' ').collect();
    println!("{}{:?}", indentation, (
        (
            tree.bbox.min.x,
            tree.bbox.min.y,
            tree.bbox.min.z,
        ),
        (
            tree.bbox.max.x,
            tree.bbox.max.y,
            tree.bbox.max.z,
        )
    ));

    if let NodeStorage::Interior(ref children) = tree.storage {
        for child in children.iter() {
            print_tree2(child, depth + 1);
        }
    }
    // if let NodeStorage::Leaf(ref children) = tree.storage {
    //     for child in children.iter() {
    //         let bbox = child.mbr().unwrap();
    //         println!("{}    {:?}", indentation, (
    //             (
    //                 bbox.min.x,
    //                 bbox.min.y,
    //                 bbox.min.z,
    //             ),
    //             (
    //                 bbox.max.x,
    //                 bbox.max.y,
    //                 bbox.max.z,
    //             )
    //         ));
    //         // print_tree2(child, depth + 1);
    //     }
    // }
}

#[cfg(debug_assertions)] #[inline]
fn instantiation_post_message<T>(tree: &RTree<T>) where T: Mbr {
    if let Some(ref root) = tree.root {
        print_tree(root, 0);
    }
}

#[cfg(not(debug_assertions))] #[inline]
fn instantiation_post_message<T>(tree: &RTree<T>) where T: Mbr {
    // if let Some(ref root) = tree.root {
    //     print_tree2(root, 0);
    // }
}

impl<T> FromIterator<T> for RTree<T> where T: Mbr {
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> RTree<T> {
        let mut rtree = RTree::new();
        for item in iterable.into_iter() {
            rtree.insert(item);
        }
        instantiation_post_message(&rtree);
        rtree
    }
}

// Iterator

pub struct Iter<'a, T> where T: Mbr+'a{
    stack: Vec<&'a RTreeNode<T>>,
    leaf_iter: Option<SliceIter<'a, BBoxItem<T>>>,
    infinites: ::std::slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> where T: Mbr+'a {
    fn new(rtree: &'a RTree<T>) -> Iter<'a, T> {
        let mut stack: Vec<&'a RTreeNode<T>> = Vec::with_capacity(512);

        let RTree { ref root, ref infinites } = *rtree;

        if let Some(ref root) = *root {
            stack.push(root);
        }
        Iter {
            stack: stack,
            leaf_iter: None,
            infinites: infinites.iter(),
        }
    }
}

pub struct IntersectIter<'a, T> where T: Mbr+'a{
    stack: Vec<&'a RTreeNode<T>>,
    leaf_iter: Option<SliceIter<'a, BBoxItem<T>>>,
    ray: &'a Ray,
    infinites: ::std::slice::Iter<'a, T>,
}

impl<'a, T> IntersectIter<'a, T> where T: Mbr+'a {
    fn new(rtree: &'a RTree<T>, ray: &'a Ray) -> IntersectIter<'a, T> {
        let mut stack: Vec<&'a RTreeNode<T>> = Vec::with_capacity(512);

        let RTree { ref root, ref infinites } = *rtree;

        if let Some(ref root) = *root {
            stack.push(root);
        }
        IntersectIter {
            stack: stack,
            leaf_iter: None,
            ray: ray,
            infinites: infinites.iter(),
        }
    }
}

impl<'a, T> Iterator for IntersectIter<'a, T> where T: Mbr+'a {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(infinite) = self.infinites.next() {
            return Some(infinite);
        }
        loop {
            let ray = self.ray;
            if let Some(leaf_iter) = self.leaf_iter.as_mut() {
                if let Some(val) = leaf_iter.filter(|x| x.bbox.intersects(ray)).next() {
                    return Some(&val.item);
                }
            }

            // iterator went empty, so we'll pop from the stack and
            // iterate on the next node's children now,
            if let Some(node) = self.stack.pop() {
                match node.storage {
                    NodeStorage::Interior(ref children) => {
                        for child in children.iter() {
                            if child.bbox.intersects(self.ray) {
                                self.stack.push(child);
                            }
                        }
                    }
                    NodeStorage::Leaf(ref items) => {
                        self.leaf_iter = Some(items.iter())
                    }
                }
            } else {
                break;
            }
        }
        None
    }
}

// Heuristic-agnostic utilities

mod common_util {
    use std::f64;
    use super::vec3::Vec3;
    use super::bbox::BBox;
    use super::Mbr;

    pub fn mbr_from_slice<T: Mbr>(iter: &[T]) -> BBox where T: Mbr {
        let mut max = Vec3 { x: f64::MIN, y: f64::MIN, z: f64::MIN };
        let mut min = Vec3 { x: f64::MAX, y: f64::MAX, z: f64::MAX };

        for item in iter.iter() {
            let bounding = item.mbr().unwrap();
            min.x = min.x.min(bounding.min.x);
            min.y = min.y.min(bounding.min.y);
            min.z = min.z.min(bounding.min.z);

            max.x = max.x.max(bounding.max.x);
            max.y = max.y.max(bounding.max.y);
            max.z = max.z.max(bounding.max.z);
        }

        BBox {
            min: min,
            max: max
        }
    }

    /// returns true if the bbox was modified
    pub fn replace_bbox(bbox: &mut BBox, new_mbr: BBox) -> bool {
        let is_changed = *bbox != new_mbr;
        *bbox = new_mbr;
        is_changed
    }

    /// returns true if the bbox was modified
    pub fn union_bbox(bbox: &mut BBox, adding: BBox) -> bool {
        let new_bbox = bbox.union(&adding);
        if new_bbox != *bbox {
            *bbox = new_bbox;
            true
        } else {
            false
        }
    }
}

use self::quad_util as impl_util;

// Heuristic-specific utilities

mod quad_util {
    use std::f64;
    use std::cmp::{Ord, Ordering};
    use super::bbox::BBox;
    use super::{RTreeNode, Mbr};

    #[derive(Debug)]
    enum PickedBox {
        Left,
        Right,
        Tie,
    }

    pub fn box_score(bbox: &BBox) -> f64 {
        bbox.volume()
    }

    /// Makes it so the first element is always the least expanding
    pub fn score_swap<T>(origin: &BBox, left: T, right: T) -> (T, T)
        where
            T: Mbr
    {
        let left_exp = expansion(origin, &left.mbr().unwrap());
        assert!(!left_exp.is_nan());

        let right_exp = expansion(origin, &right.mbr().unwrap());
        assert!(!right_exp.is_nan());

        if left_exp <= right_exp {
            (left, right)
        } else {
            (right, left)
        }
    }

    fn pick_seeds<T>(items: &[T]) -> Option<(BBox, BBox)>
        where
            T: Mbr {

        let mut max_d = f64::MIN;
        let mut best_box: Option<(BBox, BBox)> = None;

        for e1 in items.iter().map(Mbr::mbr).map(Option::unwrap) {
            for e2 in items.iter().map(Mbr::mbr).map(Option::unwrap) {
                let difference = expansion2(&e1, &e2);
                if max_d <= difference {
                    max_d = difference;
                    best_box = Some((e1, e2));
                }
            }
        }

        best_box
    }

    fn expansion(target: &BBox, adding: &BBox) -> f64 {
        target.union(adding).surface_area() - target.surface_area()
    }

    fn expansion2(e1: &BBox, e2: &BBox) -> f64 {
        e1.union(e2).surface_area() - e1.surface_area() - e2.surface_area()
    }

    pub fn split<T>(items: Vec<T>) -> (BBox, Vec<T>, BBox, Vec<T>)
        where
            T: Mbr {

        use std::io::{self, Write};
        let mut items = items;
        let (mut lbox, mut rbox) = pick_seeds(&items[..]).expect("Unsufficient nodes");

        // `items` should be the size of a full node.  Size the other two
        // similarly.
        let mut lefts = Vec::with_capacity(items.len());
        let mut rights = Vec::with_capacity(items.len());

        // TODO: we need to satisfy the minimum fill!
        let mut tie_break = false;

        let mut buf = Vec::with_capacity(4096);
        while items.len() > 0 {
            let (best_idx, which_box) = best_fit_pair(lbox, rbox, &items[..]).unwrap();

            let best = items.remove(best_idx);
            let best_bbox = best.mbr().unwrap();

            writeln!(&mut buf, "which_box = {:?}", which_box);

            match (which_box, tie_break) {
                (PickedBox::Left, _) => {
                    lbox = best_bbox.union(&lbox);
                    lefts.push(best);
                    tie_break = true;
                },
                (PickedBox::Right, _) => {
                    rbox = best_bbox.union(&rbox);
                    rights.push(best);
                    tie_break = false;
                },
                (PickedBox::Tie, false) => {
                    lbox = best_bbox.union(&lbox);
                    lefts.push(best);
                    tie_break = true;
                },
                (PickedBox::Tie, true) => {
                    rbox = best_bbox.union(&rbox);
                    rights.push(best);
                    tie_break = false;
                },
            }
        }

        if lefts.len() == 0 || rights.len() == 0 {
            let stdout_o = io::stdout();
            stdout_o.lock().write_all(&buf[..]).unwrap();
        }

        // always return the smallest box first
        if box_score(&lbox) < box_score(&rbox) {
            (lbox, lefts, rbox, rights)
        } else {
            (rbox, rights, lbox, lefts)
        }
    }

    fn best_fit_pair<T>(left: BBox, right: BBox, children: &[T]) -> Option<(usize, PickedBox)> where T: Mbr {
        if children.len() == 0 {
            return None;
        }

        let mut min_expansion = f64::MAX;
        let mut min_idx = None;
        let mut is_left = PickedBox::Tie;

        for (i, child) in children.iter().enumerate() {
            let bbox = child.mbr().unwrap();
            let left_expansion = expansion(&bbox, &left);
            let right_expansion = expansion(&bbox, &right);

            if left_expansion <= min_expansion {
                min_expansion = left_expansion;
                min_idx = Some(i);
                is_left = PickedBox::Left;
            }
            if right_expansion <= min_expansion {
                min_expansion = right_expansion;
                min_idx = Some(i);
                is_left = PickedBox::Right;
            }

            if left_expansion == right_expansion {
                is_left = PickedBox::Tie;
            }
        }

        Some((min_idx.unwrap(), is_left))
    }

    pub fn best_fit<T>(target: BBox, children: &[T]) -> Option<usize> where T: Mbr {
        if children.len() == 0 {
            return None;
        }

        let mut min_volume = f64::MAX;
        let mut best_idx = None;

        // Find the smallest node that contains our target.

        for (idx, child) in children.iter().enumerate() {
            let bbox = child.mbr().unwrap();

            let volume = bbox.surface_area();

            if volume.is_nan() {
                panic!("volume must not be NaN");
            }

            if bbox.contains(&target) && volume < min_volume {
                min_volume = volume;
                best_idx = Some(idx);
            }
        }

        if let Some(idx) = best_idx {
            return Some(idx);
        }

        // None of the candidates fully contained our target, so search for the
        // node which would expand the least, if our target was added to it.
        for (idx, child) in children.iter().enumerate() {
            let volume = child.mbr().unwrap().union(&target).surface_area();

            if volume.is_nan() {
                panic!("volume must not be NaN");
            }

            if volume < min_volume {
                min_volume = volume;
                best_idx = Some(idx);
            }
        }

        if let Some(idx) = best_idx {
            return Some(idx);
        }

        unreachable!("a node must have been selected");
    }
}

#[cfg(test)]
mod tests {
    use ::vec3::Vec3;
    use ::ray::Ray;
    use super::RTree;
    use super::test_helpers::Sphere;

    #[test]
    fn test_sphere() {
        let ray = Ray::new(Vec3::xyz(0.0, 0.0, 0.0), Vec3::xyz(1.0, 0.055, 0.00));

        let mut spheres: RTree<Sphere> = RTree::new();

        spheres.insert(Sphere::new(Vec3::xyz(100.0, 0.0, 0.0), 5.0).unwrap());
        spheres.insert(Sphere::new(Vec3::xyz(120.0, 0.0, 0.0), 15.0).unwrap());
        spheres.insert(Sphere::new(Vec3::xyz(140.0, 0.0, 0.0), 25.0).unwrap());
        spheres.insert(Sphere::new(Vec3::xyz(160.0, 0.0, 0.0), 35.0).unwrap());
        spheres.insert(Sphere::new(Vec3::xyz(180.0, 0.0, 0.0), 45.0).unwrap());
        spheres.insert(Sphere::new(Vec3::xyz(200.0, 0.0, 0.0), 55.0).unwrap());

        for object in spheres.intersect_iter(&ray) {
            println!("intersect = {:?}", object);
        }
        panic!();
    }

    #[test]
    fn test_splitting() {
        let mut spheres: RTree<Sphere> = RTree::new();

        for i in 0..256 {
            let distance = 100.0 * i as f64;
            let sp_origin = Vec3::xyz(distance, 0.0, 0.0);
            spheres.insert(Sphere::new(sp_origin, 5.0).unwrap());
        }

        spheres.assert_height();
    }
}

#[no_mangle]
pub extern "C" fn debugger() {}
