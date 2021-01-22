use crate::common::quad_tree::QuadTree;
use crate::game::location::pos::Position;
use crate::game::object::obj::GameObject;
use crate::user::user::AuthenticatedUser;
use std::collections::HashMap;
use crate::game::map::zone::Zone;

pub struct World {
    zones: HashMap<String, Zone>,
}

impl World {
    pub fn new() -> Self {
        let mut zones = HashMap::new();
        zones.insert("default".to_string(), Zone::default());
        World { zones }
    }
}
