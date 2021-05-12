use crate::game::components::connection::NetworkConnectionComponent;
use crate::game::components::input_cache::MovementInputCache;
use crate::net::data::IntermediateGamePacket;
use crate::user::user::AuthenticatedUser;
use legion::system;

#[system(for_each)]
pub fn user_input(conn: &mut NetworkConnectionComponent, input_cache: &mut MovementInputCache) {
    let conn: &mut AuthenticatedUser = &mut conn.user;
    if let Some(reader) = &mut conn.reader {
        loop {
            let data = reader.try_recv();
            if let Ok(data) = data {
                debug!("Received data from user: {} {:#?}", conn.name, &data);
                match data {
                    IntermediateGamePacket::PlayerInput { user, action } => {
                        input_cache.movements.push_back(action.clone())
                    }
                    _ => (),
                }
            } else {
                break;
            }
        }
    }
}
