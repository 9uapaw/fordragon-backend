use crate::user::user::AuthenticatedUser;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct NetworkConnectionComponent {
    pub user: AuthenticatedUser
}

impl NetworkConnectionComponent {
    pub fn new(user: AuthenticatedUser) -> Self {
        NetworkConnectionComponent { user }
    }
}