use crate::common::obj_id::GameObjectIdentifier;
use crate::game::components::input_cache::MovementInputCache;
use crate::game::components::movement::{Location, Transformation};
use crate::game::components::obj::GameObjectDescriptor;
use crate::game::components::state::{MovableStateData, StateMachineComponent};
use crate::game::location::pos::Position;
use crate::game::resource::frame::FrameResource;
use crate::game::resource::state_delta::StateDeltaCache;
use crate::net::packet::state_delta::ObjectStateDeltaPacket;
use legion::{system, Entity};
use std::borrow::BorrowMut;
use std::time::{Duration, Instant};

#[system(for_each)]
pub fn movement_control(
    #[resource] frame: &FrameResource,
    #[resource] state_delta: &mut StateDeltaCache,
    transformation: &mut Transformation,
    location: &mut Location,
    state: &mut StateMachineComponent<MovableStateData>,
    input: &mut MovementInputCache,
    entity: &Entity,
    obj: &mut GameObjectDescriptor,
) {
    debug!("Movement system handling object: {}", obj);
    let mut movable_state = MovableStateData::new(
        Some(transformation),
        Some(location),
        frame.frame_delta,
        input.movements.pop_front(),
    );
    state.update(&mut movable_state);
    if !movable_state.state_delta.as_mut().unwrap().batch.is_empty() {
        state_delta.0.push_back(ObjectStateDeltaPacket::new(
            obj.id.clone(),
            movable_state.state_delta.take().unwrap(),
        ))
    }
}
