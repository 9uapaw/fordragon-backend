use crate::common::quad_tree::QuadTree;
use crate::game::location::pos::{Area, Position};
use crate::game::object::obj::GameObject;
use std::collections::HashMap;

pub struct Zone {
    id: String,
    grid: QuadTree<GameObject>,
}

impl Zone {
    pub fn new(id: String, grid: QuadTree<GameObject>) -> Self {
        Zone { id, grid }
    }
}

impl Default for Zone {
    fn default() -> Self {
        Zone {
            id: "default".to_string(),
            grid: QuadTree::new(
                Position::from_coord(0.0, 0.0),
                Position::from_coord(1000.0, 1000.0),
                100,
                4,
            ),
        }
    }
}
