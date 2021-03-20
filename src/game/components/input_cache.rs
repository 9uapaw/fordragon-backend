use crate::net::data::PlayerInputAction;
use specs::{Component, VecStorage};
use std::collections::VecDeque;

#[derive(Component)]
#[storage(VecStorage)]
pub struct MovementInputCache {
    pub movements: VecDeque<PlayerInputAction>,
}

impl MovementInputCache {
    pub fn new() -> Self {
        MovementInputCache {
            movements: VecDeque::new(),
        }
    }
}
