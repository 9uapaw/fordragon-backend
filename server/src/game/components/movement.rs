use crate::game::location::facing::Facing;
use crate::game::location::pos::Position;
use std::time::{Instant, Duration};

#[derive(Debug)]
pub struct Location {
    pub position: Position
}

#[derive(Debug)]
pub struct Transformation {
    pub facing: Facing,
    pub speed: f32
}

pub struct MovementComponent {
    pub facing: Facing,
    pub position: Position,
    pub speed: f32,
}

impl MovementComponent {
    pub fn new(facing: Facing, position: Position, speed: f32) -> Self {
        MovementComponent {
            facing,
            position,
            speed,
        }
    }
}

impl Default for MovementComponent {
    fn default() -> Self {
        Self {
            facing: Facing::new(),
            position: Position::new(),
            speed: 0.0,
        }
    }
}
