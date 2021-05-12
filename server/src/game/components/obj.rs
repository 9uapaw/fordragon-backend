use std::fmt::{Display, Formatter};
use crate::common::obj_id::GameObjectIdentifier;

pub struct GameObjectDescriptor {
    pub id: GameObjectIdentifier,
    pub zone_id: String,
}

impl GameObjectDescriptor {
    pub fn new(id: GameObjectIdentifier, zone_id: String) -> Self {
        GameObjectDescriptor { id, zone_id }
    }
}

impl Display for GameObjectDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id)
    }
}
