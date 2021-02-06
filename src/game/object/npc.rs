use crate::game::object::obj::{StateDelta, ObservableObject};
use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::game::location::pos::{Position, Positionable};
use crate::game::components::movement::MovementComponent;

pub struct NonPlayerCharacter {
    observers: HashMap<String, mpsc::UnboundedSender<StateDelta>>,
    pub movement: MovementComponent,
}

impl ObservableObject for NonPlayerCharacter {
    fn get_observers(&mut self) -> &mut HashMap<String, mpsc::UnboundedSender<StateDelta>> {
        &mut self.observers
    }
}