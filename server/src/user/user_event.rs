use crate::user::user::AuthenticatedUser;
use std::net::SocketAddr;

pub enum UserChangeEvent {
    NewUser(AuthenticatedUser),
    DisconnectedUser(SocketAddr)
}