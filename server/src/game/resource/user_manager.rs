use crate::common::obj_id::GameObjectIdentifier;
use crate::user::user::AuthenticatedUser;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;

#[derive(Default)]
pub struct UserManagerStorage {
    pub new_users: VecDeque<AuthenticatedUser>,
    pub disconnected_users: VecDeque<SocketAddr>,
    pub socket_to_id: HashMap<SocketAddr, GameObjectIdentifier>,
}

impl UserManagerStorage {
    pub fn new() -> Self {
        UserManagerStorage {
            new_users: VecDeque::new(),
            disconnected_users: VecDeque::new(),
            socket_to_id: HashMap::new(),
        }
    }
}
