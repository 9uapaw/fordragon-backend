use crate::user::user::AuthenticatedUser;
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Default)]
pub struct NewUsers(pub VecDeque<AuthenticatedUser>);

impl Display for NewUsers {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().format(" "))
    }
}