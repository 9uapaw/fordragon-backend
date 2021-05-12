use crate::game::components::input_cache::MovementInputCache;
use crate::game::components::movement::{Location, Transformation};
use crate::game::components::state::{MovableStateData, StateMachineComponent};
use crate::game::location::facing::Facing;
use crate::game::location::pos::Position;
use crate::game::resource::frame::FrameResource;
use crate::game::resource::user_manager::UserManagerStorage;
use crate::game::resource::state_delta::StateDeltaCache;
use crate::game::resource::zones::Zones;
use crate::game::system::movement::movement_control_system;
use crate::game::system::network_stream::network_stream;
use crate::game::system::user_change::manage_users_system;
use crate::game::system::user_input::user_input_system;
use crate::user::user::AuthenticatedUser;
use crate::user::user_event::UserChangeEvent;
use crossbeam_channel::Receiver;
use legion::{Resources, Schedule, SystemBuilder, World};
use log::Level::Trace;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

const FPS: i32 = 1;

pub struct Lobby {
    world: World,
    dispatcher: Schedule,
    user_change: Receiver<UserChangeEvent>,
}

impl Lobby {
    pub fn new(user_change_notifier: Receiver<UserChangeEvent>) -> Self {
        Lobby {
            world: World::default(),
            dispatcher: Schedule::builder()
                .add_system(manage_users_system())
                .add_system(movement_control_system())
                .add_system(user_input_system())
                .build(),
            user_change: user_change_notifier,
        }
    }

    pub fn start(&mut self) {
        let mut resources = Resources::default();
        let waiting_time = 1 / FPS;
        resources.insert(FrameResource {
            frame_delta: Duration::new(waiting_time as u64, 0),
        });
        resources.insert(UserManagerStorage::new());
        resources.insert(StateDeltaCache::new());
        resources.insert(Zones::default());

        loop {
            let loop_start = Instant::now();
            {
                let user_manager = resources.get_mut::<UserManagerStorage>();
                if let Some(mut user_manager) = user_manager {
                    for change_event in self.user_change.try_iter() {
                        match change_event {
                            UserChangeEvent::NewUser(user) => {
                                user_manager.new_users.push_front(user)
                            }
                            UserChangeEvent::DisconnectedUser(user) => {
                                user_manager.disconnected_users.push_front(user)
                            }
                        }
                    }
                }
            }
            self.dispatcher.execute(&mut self.world, &mut resources);
            network_stream(
                AtomicPtr::new(&mut self.world),
                AtomicPtr::new(&mut resources),
            );
            let elapsed_frame_time = Instant::now() - loop_start;
            if elapsed_frame_time.as_secs() < waiting_time as u64 {
                sleep(Duration::from_secs(waiting_time as u64) - elapsed_frame_time)
            }
        }
    }
}
