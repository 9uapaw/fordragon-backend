use crate::game::components::components::{Movable, Transformable, Updatable};
use crate::game::components::movement::MovementComponent;
use crate::game::components::state::StateMachineComponent;
use crate::game::location::facing::Facing;
use crate::game::location::pos::{Position, Positionable};
use crate::game::object::obj::{ObservableObject, StateDelta};
use crate::user::user::AuthenticatedUser;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use std::time::Instant;
use tokio::sync::mpsc;

pub struct PlayerCharacter {
    pub name: String,
    pub user: AuthenticatedUser,
    pub state: StateMachineComponent<StateDelta>,
    pub movement: MovementComponent,
    update: Instant,
    observers: HashMap<String, mpsc::UnboundedSender<StateDelta>>,
}

impl PlayerCharacter {
    pub fn new(name: String, user: AuthenticatedUser) -> Self {
        PlayerCharacter {
            name,
            user,
            state: StateMachineComponent::new(),
            movement: MovementComponent::default(),
            update: Instant::now(),
            observers: HashMap::new(),
        }
    }

    pub fn hash(&self) -> String {
        let mut unique_id = String::new();
        unique_id.push_str(self.name.as_str());
        unique_id.push_str(self.user.name.as_str());

        return unique_id;
    }

    pub fn update(&mut self) {}
}

impl ObservableObject for PlayerCharacter {
    fn get_observers(&mut self) -> &mut HashMap<String, mpsc::UnboundedSender<StateDelta>> {
        &mut self.observers
    }
}
