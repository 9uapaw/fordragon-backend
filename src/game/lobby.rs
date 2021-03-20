use crate::game::components::movement::{Location, Transformation};
use crate::game::components::state::{StateMachineComponent, MovableStateData};
use crate::game::location::facing::Facing;
use crate::game::location::pos::Position;
use crate::game::resource::frame::FrameResource;
use crate::game::system::movement::MovementControlSystem;
use crate::user::user::AuthenticatedUser;
use log::Level::Trace;
use specs::shred::RunWithPool;
use specs::{Builder, Dispatcher, DispatcherBuilder, RunNow, World, WorldExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossbeam_channel::{Receiver};
use crate::game::resource::new_user::NewUsers;
use crate::game::components::input_cache::MovementInputCache;

const FPS: i32 = 1;

pub struct Lobby<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    new_user_notifier: Receiver<AuthenticatedUser>
}

impl<'a, 'b> Lobby<'a, 'b> {
    pub fn new(new_user_notifier: Receiver<AuthenticatedUser>) -> Self {
        Lobby {
            world: World::new(),
            dispatcher: DispatcherBuilder::default()
                .with(MovementControlSystem::new(), "movement", &[])
                .build(),
            new_user_notifier
        }
    }

    pub fn start(&mut self) {
        self.world.register::<Transformation>();
        self.world.register::<Location>();
        self.world.register::<StateMachineComponent<MovableStateData>>();
        self.world.register::<MovementInputCache>();
        let waiting_time = 1 / FPS;
        self.world.insert(FrameResource {
            frame_delta: Duration::new(waiting_time as u64, 0),
        });
        self.world.insert(NewUsers(Vec::new()));
        let entity = self
            .world
            .create_entity()
            .with(Location {
                position: Position::new(),
            })
            .with(Transformation {
                facing: Facing::new(),
                speed: 1.0,
            })
            .with(StateMachineComponent::<MovableStateData>::new())
            .with(MovementInputCache::new())
            .build();

        loop {
            let loop_start = Instant::now();
            let new_users = self.world.get_mut::<NewUsers>();
            if let Some(new_users) = new_users {
                new_users.0.extend(self.new_user_notifier.try_iter());
            }
            self.dispatcher.dispatch(&mut self.world);
            self.world.maintain();
            let elapsed_frame_time = Instant::now() - loop_start;
            if elapsed_frame_time.as_secs() < waiting_time as u64 {
                sleep(Duration::from_secs(waiting_time as u64) - elapsed_frame_time)
            }
        }
    }
}
