use crate::game::location::pos::{Position, Positionable};
use crate::game::object::character::PlayerCharacter;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub enum StateDelta {}

pub enum GameObjectDetails {
    NPC,
    PC(PlayerCharacter),
}

pub struct GameObject {
    position: Position,
    details: GameObjectDetails,
    observers: HashMap<String, mpsc::UnboundedSender<StateDelta>>,
}

impl GameObject {
    pub fn new(position: Position, details: GameObjectDetails) -> Self {
        GameObject {
            position,
            details,
            observers: HashMap::new(),
        }
    }

    pub fn register(&mut self, k: &str) -> mpsc::UnboundedReceiver<StateDelta> {
        let (tx, rx): (
            mpsc::UnboundedSender<StateDelta>,
            mpsc::UnboundedReceiver<StateDelta>,
        ) = mpsc::unbounded_channel();
        self.observers.insert(k.to_string(), tx);
        rx
    }
}

impl Positionable for GameObject {
    fn position(&self) -> Position {
        self.position
    }
}
