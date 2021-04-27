use crate::game::components::input_cache::MovementInputCache;
use crate::game::components::movement::{Location, Transformation};
use crate::game::components::state::{MovableStateData, StateMachineComponent};
use crate::game::location::facing::Facing;
use crate::game::location::pos::Position;
use crate::game::resource::frame::FrameResource;
use crate::game::resource::new_user::NewUsers;
use crate::game::system::movement::{movement_control_system};
use crate::game::system::new_user::new_user_system;
use crate::game::system::user_input::user_input_system;
use crate::user::user::AuthenticatedUser;
use crossbeam_channel::Receiver;
use legion::{Resources, Schedule, World, SystemBuilder};
use log::Level::Trace;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::ops::Deref;
use crate::game::resource::state_delta::StateDeltaCache;
use crate::game::resource::zones::Zones;
use crate::game::system::network_stream::network_stream;
use std::sync::atomic::AtomicPtr;

const FPS: i32 = 1;

pub struct Lobby {
    world: World,
    dispatcher: Schedule,
    new_user_notifier: Receiver<AuthenticatedUser>,
}

impl Lobby {
    pub fn new(new_user_notifier: Receiver<AuthenticatedUser>) -> Self {
        Lobby {
            world: World::default(),
            dispatcher: Schedule::builder()
                .add_system(new_user_system())
                .add_system(movement_control_system())
                .add_system(user_input_system())
                .build(),
            new_user_notifier,
        }
    }

    pub fn start(&mut self) {
        let mut resources = Resources::default();
        let waiting_time = 1 / FPS;
        resources.insert(FrameResource {
            frame_delta: Duration::new(waiting_time as u64, 0),
        });
        resources.insert(NewUsers(VecDeque::new()));
        resources.insert(StateDeltaCache::new());
        resources.insert(Zones::default());

        loop {
            debug!("Game loop beginning");
            let loop_start = Instant::now();
            {
                let new_users = resources.get_mut::<NewUsers>();
                if let Some(mut new_users) = new_users {
                    new_users.0.extend(self.new_user_notifier.try_iter());
                    if !new_users.0.is_empty() {
                        debug!("Received new user {}", new_users.deref());
                    }
                }
            }
            self.dispatcher.execute(&mut self.world, &mut resources);
            network_stream(AtomicPtr::new(&mut self.world), AtomicPtr::new(&mut resources));
            let elapsed_frame_time = Instant::now() - loop_start;
            if elapsed_frame_time.as_secs() < waiting_time as u64 {
                sleep(Duration::from_secs(waiting_time as u64) - elapsed_frame_time)
            }
        }
    }
}
