use crate::common::quad_tree::QuadTree;
use crate::game::location::pos::{Area, LocatableGameObject, Position};
use legion::Entity;
use std::borrow::Borrow;
use std::collections::HashMap;

pub struct Zone {
    id: String,
    pub grid: QuadTree<LocatableGameObject>,
}

impl Zone {
    pub fn new(id: String, grid: QuadTree<LocatableGameObject>) -> Self {
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

impl Zone {
    pub fn get_neighbors_of(
        &mut self,
        id: String,
    ) -> Option<&mut HashMap<String, LocatableGameObject>> {
        self.grid.find_node_of_value(&id).map(|n| n.get_values())
    }
}
