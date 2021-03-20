use crate::game::components::input_cache::MovementInputCache;
use crate::game::components::movement::{Location, Transformation};
use crate::game::components::state::{MovableStateData, StateMachineComponent};
use crate::game::location::pos::Position;
use crate::game::resource::frame::FrameResource;
use specs::{Read, ReadStorage, System, WriteStorage};
use std::borrow::BorrowMut;
use std::time::{Duration, Instant};

pub struct MovementControlSystem {}

impl MovementControlSystem {
    pub fn new() -> Self {
        MovementControlSystem {}
    }
}

impl<'a> System<'a> for MovementControlSystem {
    type SystemData = (
        Read<'a, FrameResource>,
        WriteStorage<'a, Transformation>,
        WriteStorage<'a, Location>,
        WriteStorage<'a, StateMachineComponent<MovableStateData>>,
        WriteStorage<'a, MovementInputCache>,
    );

    fn run(&mut self, (frame, mut transform, mut loc, mut state, mut input): Self::SystemData) {
        use specs::Join;

        for (transform, location, state, input) in
            (&mut transform, &mut loc, &mut state, &mut input).join()
        {
            let transformation: &mut Transformation = transform;
            let speed = transformation.speed;
            let angle = transformation.facing.get_facing();
            let delta: Duration = frame.frame_delta;
            let calculated_speed = speed * delta.as_secs() as f32;

            let vx = calculated_speed * angle.cos();
            let vy = calculated_speed * angle.sin();

            let new_position = Position::from_coord(
                location.position.x() + vx as f64,
                location.position.y() + vy as f64,
            );

            location.position = new_position;

            let mut movable_state = MovableStateData::new(
                Some(transformation),
                Some(location),
                delta,
                input.movements.pop_front(),
            );
            state.update(&mut movable_state);
        }
    }
}
