use crate::common::obj_id::GameObjectIdentifier;
use crate::game::components::connection::NetworkConnectionComponent;
use crate::game::components::input_cache::MovementInputCache;
use crate::game::components::movement::{Location, Transformation};
use crate::game::components::obj::GameObjectDescriptor;
use crate::game::components::state::{MovableStateData, StateMachineComponent};
use crate::game::location::facing::Facing;
use crate::game::location::pos::{LocatableGameObject, Position};
use crate::game::resource::new_user::NewUsers;
use crate::game::resource::state_delta::StateDeltaCache;
use crate::game::resource::zones::Zones;
use crate::net::packet::spawn::SpawnPacket;
use crate::net::packet::state_delta::{
    ObjectStateBatch, ObjectStateChange, ObjectStateDeltaPacket,
};
use legion::system;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::Query;
use std::borrow::{Borrow, BorrowMut};

#[system]
pub fn new_user(
    cmd: &mut CommandBuffer,
    #[resource] users: &mut NewUsers,
    #[resource] zones: &mut Zones,
    #[resource] state_delta: &mut StateDeltaCache,
) {
    for user in users.0.drain(0..) {
        info!("Adding new user {}", &user.name);
        let id = user.name.clone();
        let entity = cmd.push((
            Location {
                position: Position::new(),
            },
            Transformation {
                speed: 1.0,
                facing: Facing::new(),
            },
            NetworkConnectionComponent::new(user),
            StateMachineComponent::<MovableStateData>::new(),
            MovementInputCache::new(),
        ));
        let obj_id = GameObjectIdentifier::new(entity, id.clone());
        cmd.add_component(entity, GameObjectDescriptor::new(obj_id.clone(), "1".to_string()));
        if let Some(zone) = zones.zones.get_mut("1") {
            zone.grid.add(
                id.clone(),
                LocatableGameObject::new(obj_id.clone(), Position::new()),
            );
            info!("Added {} to zone {}", &obj_id, 1);
            let mut obj_state = ObjectStateBatch::new();
            obj_state.add(ObjectStateChange::Spawn(SpawnPacket::new(
                id.clone(),
                Position::new(),
            )));
            state_delta
                .0
                .push_back(ObjectStateDeltaPacket::new(obj_id.clone(), obj_state));
        }
    }
}
