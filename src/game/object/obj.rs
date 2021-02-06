use crate::game::components::components::Transformable;
use crate::game::location::facing::Facing;
use crate::game::location::pos::{Position, Positionable};
use crate::game::object::npc::NonPlayerCharacter;
use crate::game::object::pc::PlayerCharacter;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::mpsc;

pub trait ObservableObject {
    #[inline]
    fn get_observers(&mut self) -> &mut HashMap<String, mpsc::UnboundedSender<StateDelta>>;

    fn notify(&mut self, delta: StateDelta) {
        for observer in self.get_observers() {
            observer.1.send(delta.clone());
        }
    }

    fn register(&mut self, k: &str, tx: mpsc::UnboundedSender<StateDelta>) {
        self.get_observers().insert(k.to_string(), tx);
    }
}

#[derive(Clone)]
pub enum StateDelta {}

pub enum GameObjectDetails {
    Pc(PlayerCharacter),
    Npc(NonPlayerCharacter),
}

pub struct GameObject {
    pub details: GameObjectDetails,
}

impl GameObject {
    pub fn new(details: GameObjectDetails) -> Self {
        GameObject { details }
    }
}

impl Positionable for GameObject {
    fn position(&self) -> Position {
        match &self.details {
            GameObjectDetails::Pc(pc) => pc.movement.position,
            GameObjectDetails::Npc(npc) => npc.movement.position,
        }
    }
}
