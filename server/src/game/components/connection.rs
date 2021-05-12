use crate::user::user::AuthenticatedUser;

pub struct NetworkConnectionComponent {
    pub user: AuthenticatedUser
}

impl NetworkConnectionComponent {
    pub fn new(user: AuthenticatedUser) -> Self {
        NetworkConnectionComponent { user }
    }
}