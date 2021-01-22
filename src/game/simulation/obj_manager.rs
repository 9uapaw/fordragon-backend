use crate::common::quad_tree::QuadTree;
use crate::game::object::obj::GameObject;
use std::sync::{Arc, Mutex};

pub struct ObjectManager {
    zone: Arc<Mutex<QuadTree<GameObject>>>
}

impl ObjectManager {
    pub fn new(zone: Arc<Mutex<QuadTree<GameObject>>>) -> Self {
        ObjectManager { zone }
    }

    pub fn update(&mut self) {
        let mut zone_lock = self.zone.lock();
        if let Ok(z) = zone_lock {

        }
    }
}