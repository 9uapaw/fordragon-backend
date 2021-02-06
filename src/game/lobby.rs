use crate::game::map::world::World;
use std::sync::{Arc, Mutex};
use crate::user::user::AuthenticatedUser;
use std::collections::HashMap;

pub type SafeWorld = Arc<Mutex<World>>;

pub struct Lobby {
    world: SafeWorld,
    users: HashMap<String, AuthenticatedUser>
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            world: Arc::new(Mutex::new(World::new())),
            users: HashMap::new()
        }
    }

    pub async fn start(&self) {
        let world = self.world.clone();
        tokio::spawn(async move {
            world;
        }).await;
    }

    pub fn enter_user(&mut self, user: AuthenticatedUser) {
        info!("User {} entered world", &user);

    }
}
