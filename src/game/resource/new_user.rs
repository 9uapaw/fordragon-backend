use crate::user::user::AuthenticatedUser;

#[derive(Default)]
pub struct NewUsers(pub Vec<AuthenticatedUser>);