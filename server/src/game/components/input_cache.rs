use crate::net::data::PlayerInputAction;
use std::collections::VecDeque;

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
