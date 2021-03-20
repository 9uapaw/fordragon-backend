use crate::game::components::connection::NetworkConnectionComponent;
use crate::game::components::input_cache::MovementInputCache;
use specs::{System, WriteStorage};
use crate::user::user::AuthenticatedUser;
use crate::net::data::IntermediateGameData;

pub struct NetworkUserInputSystem {}

impl<'a> System<'a> for NetworkUserInputSystem {
    type SystemData = (
        WriteStorage<'a, NetworkConnectionComponent>,
        WriteStorage<'a, MovementInputCache>,
    );

    fn run(&mut self, (mut conn, mut input_cache): Self::SystemData) {
        use specs::Join;
        for (conn, input_cache) in (&mut conn, &mut input_cache).join() {
            let conn: &mut AuthenticatedUser = &mut conn.user;
            if let Some(reader) = &mut conn.reader {
                loop {
                    let data = reader.try_recv();
                    if let Ok(data) = data {
                        match data {
                            IntermediateGameData::PlayerInput{user, action} =>
                                input_cache.movements.push_back(action.clone()),
                            _ => (),
                        }
                    } else {
                        break;
                    }
                }
            }

        }
    }
}
