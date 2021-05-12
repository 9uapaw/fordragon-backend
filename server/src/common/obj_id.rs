use legion::Entity;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct GameObjectIdentifier {
    pub internal: Entity,
    pub external: String,
}

impl GameObjectIdentifier {
    pub fn new(internal: Entity, external: String) -> Self {
        GameObjectIdentifier { internal, external }
    }
}

impl Display for GameObjectIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:#?} | {}]", self.internal, self.external)
    }
}
