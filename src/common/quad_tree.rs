#![feature(drain_filter)]

use crate::game::location::pos::{Area, Position, Positionable};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::slice::{Iter, IterMut};
use std::sync::atomic::AtomicPtr;
use crate::game::object::obj::GameObject;

#[derive(Debug)]
pub struct QuadNode<T>
where
    T: Positionable,
{
    area: Area,
    children: Vec<usize>,
    values: HashMap<String, T>,
    depth: usize,
    index: usize,
    tree: AtomicPtr<QuadTree<T>>,
}

impl<T> QuadNode<T>
where
    T: Positionable,
{
    pub fn new(
        min: Position,
        max: Position,
        depth: usize,
        index: usize,
        tree: *mut QuadTree<T>,
    ) -> Self {
        QuadNode {
            area: Area::from_point((min.x(), min.y()), (max.x(), max.y())),
            children: Vec::new(),
            values: HashMap::new(),
            depth,
            index,
            tree: AtomicPtr::new(tree),
        }
    }

    #[inline]
    pub fn third_quadrant(&mut self) -> Option<&mut QuadNode<T>> {
        let index = self.children.get(0);
        if let Some(i) = index {
            return unsafe { (*(*self.tree.get_mut())).arena.get_mut(*i) };
        }

        None
    }

    #[inline]
    pub fn second_quadrant(&mut self) -> Option<&mut QuadNode<T>> {
        let index = self.children.get(1);
        if let Some(i) = index {
            return unsafe { (*(*self.tree.get_mut())).arena.get_mut(*i) };
        }

        None
    }

    #[inline]
    pub fn first_quadrant(&mut self) -> Option<&mut QuadNode<T>> {
        let index = self.children.get(2);
        if let Some(i) = index {
            return unsafe { (*(*self.tree.get_mut())).arena.get_mut(*i) };
        }

        None
    }

    #[inline]
    pub fn fourth_quadrant(&mut self) -> Option<&mut QuadNode<T>> {
        let index = self.children.get(3);
        if let Some(i) = index {
            return unsafe { (*(*self.tree.get_mut())).arena.get_mut(*i) };
        }

        None
    }

    pub fn add_value(&mut self, k: String, v: T) {
        if !self.is_leaf() {
            return;
        }

        self.values.insert(k, v);
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

    pub fn get_values(&mut self) -> &mut HashMap<String, T> {
        &mut self.values
    }
}

#[derive(Debug)]
pub struct QuadTree<T>
where
    T: Positionable,
{
    arena: Vec<QuadNode<T>>,
    root: usize,
    bucket_size: usize,
    max_depth: usize,
    object_node_map: HashMap<String, usize>,
}

impl<T> QuadTree<T>
where
    T: Positionable,
{
    pub fn new(min: Position, max: Position, bucket_size: usize, max_depth: usize) -> Self {
        let mut tree = QuadTree {
            arena: Vec::new(),
            root: 0,
            bucket_size,
            max_depth,
            object_node_map: HashMap::new(),
        };
        let mut tree_ptr = &mut tree as *mut QuadTree<T>;
        tree.arena.push(QuadNode::new(min, max, 0, 0, tree_ptr));
        tree
    }

    pub fn get_root(&mut self) -> &mut QuadNode<T> {
        self.arena.get_mut(self.root).unwrap()
    }

    pub fn find(&mut self, k: String) -> Option<&mut T> {
        let node_id = self.object_node_map.get(&k);
        if let Some(n) = node_id {
            if let Some(node) = self.arena.get_mut(*n) {
                return node.get_value(k.clone());
            }
        }

        None
    }

    pub fn add(&mut self, k: String, v: T) {
        let mut node_candidate_index = self.root;
        loop {
            let node_candidate = self.arena.get(node_candidate_index).unwrap();
            if node_candidate.is_leaf() {
                break;
            }
            for child_index in &node_candidate.children {
                let mut child = self.arena.get(*child_index);
                if let Some(child) = child {
                    if child.get_area().contains(v.position()) {
                        node_candidate_index = *child_index;
                    }
                }
            }
        }

        let candidate_id = self.try_split(node_candidate_index, &v);
        let mut node_candidate = self.arena.get_mut(candidate_id);
        if let Some(node) = node_candidate {
            node.add_value(k.clone(), v);
            self.object_node_map.insert(k, candidate_id);
        }
    }

    pub fn remove(&mut self, k: &str) {
        let mut removed = false;
        if let Some(n) = self.find_node_of_value(k) {
            n.values.remove(k);
            removed = true;
        }
        if removed {
            self.object_node_map.remove(k);
        }
    }

    pub fn find_node_of_value(&mut self, k: &str) -> Option<&mut QuadNode<T>> {
        if let Some(node_index) = self.object_node_map.get(k) {
            self.arena.get_mut(*node_index)
        } else {
            None
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<QuadNode<T>> {
        self.arena.iter_mut()
    }

    pub fn iter(&self) -> Iter<QuadNode<T>> {
        self.arena.iter()
    }

    fn try_split(&mut self, node_id: usize, v: &T) -> usize {
        let mut new_children = Vec::new();
        let current_max_node_id = self.arena.len() - 1;
        let mut selected_node_id = node_id;
        let mut values: HashMap<String, T> = HashMap::new();
        let mut tree_ptr = self as *mut QuadTree<T>;
        let mut node = self.arena.get_mut(node_id);
        if let Some(node) = node {
            if node.values.len() >= self.bucket_size && node.depth < self.max_depth {
                let min_x = node.area.min().x();
                let min_y = node.area.min().y();
                let max_x = node.area.max().x();
                let max_y = node.area.max().y();

                let avg_x = ((node.area.min().x() + node.area.max().x()) / 2 as f64).floor();
                let avg_y = ((node.area.min().y() + node.area.max().y()) / 2 as f64).floor();
                values.extend(node.values.drain());

                new_children.push(QuadNode::new(
                    Position::from_coord(min_x, min_y),
                    Position::from_coord(avg_x, avg_y),
                    node.depth + 1,
                    current_max_node_id + 1,
                    tree_ptr.clone(),
                ));
                new_children.push(QuadNode::new(
                    Position::from_coord(min_x, avg_y),
                    Position::from_coord(avg_x, max_y),
                    node.depth + 1,
                    current_max_node_id + 2,
                    tree_ptr.clone(),
                ));
                new_children.push(QuadNode::new(
                    Position::from_coord(avg_x, avg_y),
                    Position::from_coord(max_x, max_y),
                    node.depth + 1,
                    current_max_node_id + 3,
                    tree_ptr.clone(),
                ));
                new_children.push(QuadNode::new(
                    Position::from_coord(avg_x, min_y),
                    Position::from_coord(max_x, avg_y),
                    node.depth + 1,
                    current_max_node_id + 4,
                    tree_ptr.clone(),
                ));
                node.children
                    .extend(current_max_node_id + 1..=current_max_node_id + 4);
            }
        }
        let keys: Vec<String> = values.keys().map(|k| k.clone()).collect();
        for child in &mut new_children {
            if child.get_area().contains(v.position()) {
                selected_node_id = child.index;
            }
            for object_id in &keys {
                let value = values.get(object_id);
                let mut found = false;
                if let Some(v) = value {
                    if child.get_area().contains(v.position()) {
                        found = true;
                    }
                }
                drop(value);
                if found {
                    child.add_value(object_id.clone(), values.remove(object_id).unwrap());
                    self.object_node_map.insert(object_id.clone(), child.index);
                }
            }
        }
        self.arena.extend(new_children);
        selected_node_id
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
        {
            let mut third = root.third_quadrant().unwrap();
            assert_eq!(third.values.len(), 1);
        }
        {
            let mut second = root.second_quadrant().unwrap();
            assert_eq!(second.values.len(), 2);
        }
        {
            let mut first = root.first_quadrant().unwrap();
            assert_eq!(first.values.len(), 2);
        }
        let mut fourth = root.fourth_quadrant().unwrap();
        assert_eq!(fourth.values.len(), 1);
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

        tree.remove("3");

        let mut element = tree.find("3".to_string());
        if let Some(e) = element {
            panic!("Element is found, but it was removed before");
        }
    }
}
