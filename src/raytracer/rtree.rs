#![allow(dead_code)]
use geometry::{BBox, Prim};
use vec3::Vec3;
use std::default::Default;

// References:
//  * http://www.cs.sfu.ca/CourseCentral/454/jpei/slides/R-Tree.pdf

static ZERO_BOX: BBox = BBox {
    min: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    max: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[must_use]
enum InsertionResult {
    None,
    Split,
}

enum NodeData<T, S>
    where T: Prim+Send+Sync,
          S: RTreeStrategies<T>
{
    Interior(Vec<RTreeNode<T, S>>),
    Leaf(Vec<T>),
}

struct RTreeNodeEntry<I> {
    bbox: BBox,
    item: I
}

struct RTreeNode<T, S>
    where T: Prim+Send+Sync,
          S: RTreeStrategies<T>
{
    bbox: BBox,
    data: NodeData<T, S>,
}

impl<T, S> RTreeNode<T, S>
    where T: Prim+Send+Sync,
          S: RTreeStrategies<T>
{
    fn new(value: T) -> RTreeNode<T, S> {
        RTreeNode {
            bbox: value.partial_bounding().unwrap(),
            data: NodeData::Leaf(vec![value])
        }
    }

    fn new_interior<I>(iter: I) -> Option<RTreeNode<T, S>>
        where I: Iterator<RTreeNode<T, S>> {
        let childs = iter.collect::<Vec<_>>();
        if 0 < childs.len() {
            let union_box = childs[1..]
                .iter().map(|c| c.bbox)
                .fold(childs[0].bbox, BBox::union);

            Some(RTreeNode {
                bbox: union_box,
                data: NodeData::Interior(childs)
            })
        } else {
            None
        }
    }

    fn insert(&mut self, strategies: &S, value: T) -> InsertionResult {
        let target_bbox = value.partial_bounding().unwrap();
        assert!(self.bbox.overlaps(&target_bbox));

        match self.data {
            NodeData::Interior(ref mut children) => {
                let new_node = {
                    let insertion_node = strategies.select_child(&mut children[], &value);
                    match insertion_node.insert(strategies, value) {
                        InsertionResult::Split => {
                            // insertion_node must be split now.
                            Some(strategies.split_node(insertion_node))
                        },
                        InsertionResult::None => None
                    }
                };
                match new_node {
                    Some(node) => {
                        children.push(node);
                        match strategies.node_max() < children.len() {
                            true => InsertionResult::Split,
                            false => InsertionResult::None,
                        }
                    },
                    None => InsertionResult::None
                }
            },
            NodeData::Leaf(ref mut prims) => {
                prims.push(value);
                match strategies.node_max() < prims.len() {
                    true => InsertionResult::Split,
                    false => InsertionResult::None,
                }
            }
        }
    }
}

pub struct RTree<T, S=QuadraticStrategy> {
    infinites: Vec<T>,
    root: Option<RTreeNode<T, S>>,
    strategies: S,
}

impl<T, S> RTree<T, S> where T: Prim+Send+Sync, S: RTreeStrategies<T> {
    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(RTreeNode::new(value));
            return;
        }
        
        match self.root.as_mut().unwrap().insert(&self.strategies, value) {
            // root node must be split now.
            InsertionResult::Split => {
                let mut old_root = self.root.take().unwrap();
                let new_node = self.strategies.split_node(&mut old_root);

                let new_root = RTreeNode::new_interior([old_root, new_node].into_iter());
                self.root = Some(new_root.unwrap());
            },
            InsertionResult::None => ()
        };
    }
    
    pub fn search<'a, F>(&'a self, f: F) -> RTreeIterator<'a, T, S>
        where F: Fn(BBox) -> bool + 'static
    {
        RTreeIterator::new(self, Box::new(f))
    }

    pub fn intersect<'a>(&'a self, bbox: BBox) -> RTreeIterator<'a, T, S> {
        self.search(move |candidate: BBox| candidate.overlaps(&bbox))
    }

}

impl<T> RTree<T, QuadraticStrategy>
    where T: Prim+Send+Sync
{
    pub fn new() -> RTree<T, QuadraticStrategy> {
        // RTreeNode::zero()
        RTree {
            infinites: Vec::new(),
            root: None,
            strategies: Default::default(),
        }
    }
}


pub trait RTreeStrategies<T>
    where T: Prim+Send+Sync
{
    fn node_min(&self) -> usize;
    fn node_max(&self) -> usize;
    fn select_child<'a>(&self, &'a mut [RTreeNode<T, Self>], &T) -> &'a mut RTreeNode<T, Self>;
    fn split_node(&self, node: &mut RTreeNode<T, Self>) -> RTreeNode<T, Self>;
}

pub struct LinearStrategy {
    node_min: usize,
    node_max: usize,
}

impl Default for LinearStrategy {
    fn default() -> LinearStrategy {
        LinearStrategy {
            // These are made up.
            node_min: 10,
            node_max: 30,
        }
    }
}

fn select_minvol_child<'a, T, S>(children: &'a mut [RTreeNode<T, S>], value: &T)
    -> &'a mut RTreeNode<T, S>
    where T: Prim+Send+Sync,
          S: RTreeStrategies<T> {

    let new_bbox = value.partial_bounding().expect("Value must be Bounding");

    let mut min_val = ::std::f64::MAX_VALUE;
    let mut node = None;

    for child in children.iter_mut() {
        if child.bbox.union(&new_bbox).volume() < min_val {
            node = Some(child)
        }
    }
    node.expect("Expected children in RTreeNode, got none.")
}

impl<T> RTreeStrategies<T> for LinearStrategy where T: Prim+Send+Sync {
    fn node_min(&self) -> usize {
        self.node_min
    }

    fn node_max(&self) -> usize {
        self.node_max
    }

    fn select_child<'a>(&self, children: &'a mut [RTreeNode<T, LinearStrategy>], value: &T)
            -> &'a mut RTreeNode<T, LinearStrategy> {
        select_minvol_child(children, value)
    }

    fn split_node(&self, node: &mut RTreeNode<T, Self>) -> RTreeNode<T, LinearStrategy> {
        unimplemented!();
    }
}

pub struct QuadraticStrategy {
    node_min: usize,
    node_max: usize,
}

impl Default for QuadraticStrategy {
    fn default() -> QuadraticStrategy {
        QuadraticStrategy {
            // These are made up.
            node_min: 10,
            node_max: 30,
        }
    }
}

impl<T> RTreeStrategies<T> for QuadraticStrategy where T: Prim+Send+Sync {
    fn node_min(&self) -> usize {
        self.node_min
    }

    fn node_max(&self) -> usize {
        self.node_max
    }

    fn select_child<'a>(&self, children: &'a mut [RTreeNode<T, QuadraticStrategy>], value: &T)
            -> &'a mut RTreeNode<T, QuadraticStrategy> {
        select_minvol_child(children, value)
    }

    fn split_node(&self, node: &mut RTreeNode<T, Self>) -> RTreeNode<T, QuadraticStrategy> {
        // maximize(volume(mbb(e[1], e[2])) - volume(e[1]) - volume(e[2]))
        match node.data {
            NodeData::Interior(children) => {
                let max_pair = None;
                let max_val = -1.0;
                for (idx_a, child_a) in children.iter().enumerate() {                   
                    for (idx_b, child_b) in children.iter().enumerate() {
                        if idx_b <= idx_a {
                            continue;
                        }

                        let mmb_vol = (
                            child_a.bbox.union(child_b.bbox).volume()
                            - child_a.bbox.volume()
                            - child_b.bbox.volume());

                        if max_val < mmb_vol {
                            max_pair = Some((idx_a, idx_b)); 
                            max_val = mmb_vol;
                        }
                    }
                }
                let (idx_a, idx_b) = max_pair.unwrap();
                let child_b = children.swap_remove(idx_b);
                let child_a = children.swap_remove(idx_a);

                RTreeNode::new_interior([child_a, child_b].into_iter());
            },
            NodeData::Leaf(leaf_data) => {
                //
            },
        }
        unimplemented!();
    }
}

type SearchFunction<'a> = Fn(BBox) -> bool + 'a;

struct RTreeIterator<'a, T: 'a, S: 'a> {
    stack: Vec<&'a RTreeNode<T, S>>,
    cur_prim_iter: Option<::std::slice::Iter<'a, T>>,

    search: Box<SearchFunction<'static>>,
}

impl<'a, T, S> RTreeIterator<'a, T, S>
    where T: Prim+Send+Sync,
          S: RTreeStrategies<T>
{
    fn new(tree: &'a RTree<T, S>, search: Box<SearchFunction<'static>>) -> RTreeIterator<'a, T, S> {
        let starting_stack = match tree.root {
            Some(ref root) => vec![root],
            None => Vec::new(),
        };

        RTreeIterator {
            stack: starting_stack,
            // Iterate infinites first
            cur_prim_iter: Some(tree.infinites.iter()),
            search: search,
        }
    }
}

impl<'a, T, S> Iterator for RTreeIterator<'a, T, S>
    where T: Prim+Send+Sync+'a,
          S: RTreeStrategies<T>+'a
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            if self.stack.len() == 0 && self.cur_prim_iter.is_none() {
                return None
            }

            if let Some(mut prim_iter) = self.cur_prim_iter.take() {
                if let Some(item) = prim_iter.next() {
                    self.cur_prim_iter = Some(prim_iter);
                    return Some(item);
                }
            }

            assert!(self.cur_prim_iter.is_none());

            let deepest_node = self.stack.pop().unwrap();
            match deepest_node.data {
                NodeData::Leaf(ref items) => {
                    self.cur_prim_iter = Some(items.iter());
                    continue;
                },
                NodeData::Interior(ref nodes) => {
                    for node in nodes.iter() {
                        if self.search.call((node.bbox, )) {
                            self.stack.push(node);
                        }
                    }
                }
            }
        }
    }
}