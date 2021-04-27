use crate::game::components::connection::NetworkConnectionComponent;
use crate::game::components::obj::GameObjectDescriptor;
use crate::game::location::pos::LocatableGameObject;
use crate::game::resource::state_delta::StateDeltaCache;
use crate::game::resource::zones::Zones;
use crate::net::protocol::encode::BBEncodable;
use bytes::BytesMut;
use legion::world::SubWorld;
use legion::{system, EntityStore, Resources, World};
use std::sync::atomic::AtomicPtr;

pub fn network_stream(mut world: AtomicPtr<World>, mut resources: AtomicPtr<Resources>) {
    unsafe {
        let mut resources = &mut (*(*resources.get_mut()));
        let mut world = &mut (*(*world.get_mut()));

        if let (Some(mut state_delta), Some(mut zones)) = (
            resources.get_mut::<StateDeltaCache>(),
            resources.get_mut::<Zones>(),
        ) {
            for delta in state_delta.0.drain(0..) {
                let mut neighbours = None;
                if let Ok(mut entity) = world.entry_mut(delta.id.internal) {
                    if let Ok(obj) = entity.get_component::<GameObjectDescriptor>() {
                        if let Some(zone) = zones.zones.get_mut(&obj.zone_id) {
                            neighbours = zone
                                .get_neighbors_of(obj.id.external.clone())
                                .map(|n| n.values());
                        }
                    }
                }
                if let Some(neighbours) = neighbours {
                    for neighbour in neighbours {
                        if let Ok(mut entity) = world.entry_mut(neighbour.id.internal) {
                            if let Ok(network_connection) =
                                entity.get_component_mut::<NetworkConnectionComponent>()
                            {
                                if let Some(writer) = &mut network_connection.user.writer {
                                    let mut buf = BytesMut::new();
                                    delta.encode_as_bbp(&mut buf);
                                    debug!("{:#?}", &mut buf);
                                    writer.send(buf.freeze());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
