use crate::common::quad_tree::QuadTree;
use crate::game::location::pos::{LocatableGameObject, Position};
use crate::game::map::zone::Zone;
use std::collections::HashMap;

pub struct Zones {
    pub zones: HashMap<String, Zone>,
}

impl Default for Zones {
    fn default() -> Self {
        let mut zones = HashMap::new();
        zones.insert(
            "1".to_string(),
            Zone::new(
                "1".to_string(),
                QuadTree::new(Position::new(), Position::from_coord(1000.0, 1000.0), 50, 4),
            ),
        );

        Zones { zones }
    }
}
