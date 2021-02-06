use crate::game::location::pos::{Positionable, Position};
use crate::game::location::facing::Facing;
use std::time::Instant;

pub trait Transformable: Positionable {
    fn set_position(&mut self, new_position: Position);
    fn get_facing(&self) -> Facing;
}

pub trait Movable {
    fn get_speed(&self) -> f32;
}

pub trait Updatable {
    fn last_update(&self) -> Instant;
}
