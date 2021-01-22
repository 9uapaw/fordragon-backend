#![feature(drain_filter)]

use crate::game::location::pos::{Area, Position, Positionable};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;

#[derive(Debug)]
pub struct QuadNode<T>
where
    T: Positionable,
{
    area: Area,
    children: Vec<Box<QuadNode<T>>>,
    values: HashMap<String, T>,
    depth: usize,
}

impl<T> QuadNode<T>
where
    T: Positionable,
{
    pub fn new(min: Position, max: Position, depth: usize) -> Self {
        QuadNode {
            area: Area::from_point((min.x(), min.y()), (max.x(), max.y())),
            children: Vec::new(),
            values: HashMap::new(),
            depth,
        }
    }

    pub fn add_value(&mut self, k: String, v: T) {
        if !self.is_leaf() {
            return;
        }

        self.values.insert(k, v);
    }

    pub fn get_node_candidate(&mut self, compare: &T) -> Option<&mut QuadNode<T>> {
        if !self.area.contains(compare.position()) {
            return None;
        }

        let mut res = None;
        if self.children.is_empty() {
            res = Some(self);
        } else {
            for node in self.children.iter_mut() {
                let mut inner_res = node.get_node_candidate(compare);
                if inner_res.is_some() {
                    res = inner_res;
                }
            }
        }

        res
    }

    pub fn get_value(&mut self, k: String) -> Option<&mut T> {
        self.values.get_mut(&k)
    }

    pub fn get_area(&self) -> &Area {
        &self.area
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug)]
pub struct QuadTree<T>
where
    T: Positionable,
{
    root: Box<QuadNode<T>>,
    bucket_size: usize,
    max_depth: usize,
    object_node_map: HashMap<String, AtomicPtr<QuadNode<T>>>,
}

impl<T> QuadTree<T>
where
    T: Positionable,
{
    pub fn new(min: Position, max: Position, bucket_size: usize, max_depth: usize) -> Self {
        QuadTree {
            root: Box::new(QuadNode::new(min, max, 0)),
            bucket_size,
            max_depth,
            object_node_map: HashMap::new(),
        }
    }

    pub fn get_root(&mut self) -> &mut QuadNode<T> {
        &mut self.root
    }

    pub fn find(&mut self, k: String) -> Option<&mut T> {
        let mut ptr = self.object_node_map.get_mut(&k);
        if let Some(ref mut ptr) = ptr {
            unsafe {
                return (*(*ptr.get_mut())).values.get_mut(&k);
            }
        }

        None
    }

    pub fn add(&mut self, k: String, v: T) {
        if let Some(ref mut node) = self.root.get_node_candidate(&v) {
            if node.values.len() >= self.bucket_size && node.depth != self.max_depth {
                <QuadTree<T>>::split_node(*node);

                let mut values: HashMap<String, T> = node.values.drain().collect();
                let keys = values.keys().cloned().collect::<Vec<String>>();
                for key in keys {
                    let mut val = values.remove(&key).unwrap();
                    let pos = val.position();
                    let mut found_child = node
                        .children
                        .iter_mut()
                        .filter(|c| c.get_area().contains(pos.clone()))
                        .next();
                    if let Some(child) = found_child {
                        self.object_node_map
                            .insert(key.clone(), AtomicPtr::new(child.as_mut()));
                        child.add_value(key.clone(), val);
                    }
                }
                let candidate = node.get_node_candidate(&v);
                let node_ptr: Option<*mut QuadNode<T>> = candidate.map(|n| {
                    n.add_value(k.clone(), v);
                    n as *mut QuadNode<T>
                });
                if let Some(ptr) = node_ptr {
                    self.object_node_map.insert(k.clone(), AtomicPtr::new(ptr));
                }
            } else {
                node.add_value(k.clone(), v);
                self.object_node_map
                    .insert(k.clone(), AtomicPtr::new(*node));
            }
        }
    }

    pub fn remove(&mut self, k: String) {
        let mut node = self.object_node_map.remove(&k);
        if let Some(ref mut n) = node {
            unsafe {
                (*(*n.get_mut())).values.remove(&k);
            }
        }
    }

    fn split_node(node: &mut QuadNode<T>) {
        let min_x = node.area.min().x();
        let min_y = node.area.min().y();
        let max_x = node.area.max().x();
        let max_y = node.area.max().y();

        let avg_x = ((node.area.min().x() + node.area.max().x()) / 2 as f64).floor();
        let avg_y = ((node.area.min().y() + node.area.max().y()) / 2 as f64).floor();

        node.children.push(Box::new(QuadNode::new(
            Position::from_coord(min_x, min_y),
            Position::from_coord(avg_x, avg_y),
            node.depth + 1,
        )));
        node.children.push(Box::new(QuadNode::new(
            Position::from_coord(min_x, avg_y),
            Position::from_coord(avg_x, max_y),
            node.depth + 1,
        )));
        node.children.push(Box::new(QuadNode::new(
            Position::from_coord(avg_x, avg_y),
            Position::from_coord(max_x, max_y),
            node.depth + 1,
        )));
        node.children.push(Box::new(QuadNode::new(
            Position::from_coord(avg_x, min_y),
            Position::from_coord(max_x, avg_y),
            node.depth + 1,
        )));
    }
}

#[cfg(test)]
mod tests {
    use crate::common::quad_tree::QuadTree;
    use crate::game::location::pos::{Position, Positionable};

    #[derive(Debug)]
    pub struct TestPosition {
        pub x: f64,
        pub y: f64,
    }

    impl Positionable for TestPosition {
        fn position(&self) -> Position {
            Position::from_coord(self.x, self.y)
        }
    }

    #[test]
    fn test_split_after_bucket_size_limit_reached() {
        let mut tree: QuadTree<TestPosition> = QuadTree::new(
            Position::from_coord(0 as f64, 0 as f64),
            Position::from_coord(1000 as f64, 1000 as f64),
            5,
            4,
        );
        tree.add("1".to_string(), TestPosition { x: 0.0, y: 0.0 });
        tree.add("2".to_string(), TestPosition { x: 500.0, y: 500.0 });
        tree.add("3".to_string(), TestPosition { x: 700.0, y: 300.0 });
        tree.add("4".to_string(), TestPosition { x: 200.0, y: 600.0 });
        tree.add("5".to_string(), TestPosition { x: 100.0, y: 900.0 });

        assert!(tree.get_root().is_leaf());
        assert_eq!(tree.get_root().children.len(), 0);

        tree.add("6".to_string(), TestPosition { x: 900.0, y: 900.0 });

        let mut root = tree.get_root();
        assert!(!root.is_leaf());
        let mut third = root.children.get(0);
        let mut second = root.children.get(1);
        let mut first = root.children.get(2);
        let mut fourth = root.children.get(3);
        assert_eq!(third.unwrap().values.len(), 1);
        assert_eq!(second.unwrap().values.len(), 2);
        assert_eq!(first.unwrap().values.len(), 2);
        assert_eq!(fourth.unwrap().values.len(), 1);
    }

    #[test]
    fn test_find_element() {
        let mut tree: QuadTree<TestPosition> = QuadTree::new(
            Position::from_coord(0 as f64, 0 as f64),
            Position::from_coord(1000 as f64, 1000 as f64),
            5,
            4,
        );
        tree.add("1".to_string(), TestPosition { x: 0.0, y: 0.0 });
        tree.add("2".to_string(), TestPosition { x: 500.0, y: 500.0 });
        tree.add("3".to_string(), TestPosition { x: 700.0, y: 300.0 });

        let mut element = tree.find("3".to_string());
        if let Some(e) = element {
            assert_eq!(700.0, e.x);
            assert_eq!(300.0, e.y);
        } else {
            panic!("Failed to find element");
        }
    }

    #[test]
    fn test_remove_element() {
        let mut tree: QuadTree<TestPosition> = QuadTree::new(
            Position::from_coord(0 as f64, 0 as f64),
            Position::from_coord(1000 as f64, 1000 as f64),
            5,
            4,
        );
        tree.add("1".to_string(), TestPosition { x: 0.0, y: 0.0 });
        tree.add("2".to_string(), TestPosition { x: 500.0, y: 500.0 });
        tree.add("3".to_string(), TestPosition { x: 700.0, y: 300.0 });

        tree.remove("3".to_string());

        let mut element = tree.find("3".to_string());
        if let Some(e) = element {
            panic!("Element is found, but it was removed before");
        }
    }
}
