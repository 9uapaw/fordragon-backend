use crate::game::location::facing::Facing;
use crate::game::location::pos::Position;
use std::time::{Instant, Duration};

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

    // pub fn move_forward(&mut self, delta: Duration) {
    //     let speed = self.speed;
    //     let angle = self.facing.get_facing();
    //     let calculated_speed = speed * delta.as_secs() as f32;
    //
    //     let vx = calculated_speed * angle.cos();
    //     let vy = calculated_speed * angle.sin();
    //
    //     let new_position = Position::from_coord(
    //         self.position().x() + vx as f64,
    //         self.position().y() + vy as f64,
    //     );
    //
    //     self.position = new_position;
    // }
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
