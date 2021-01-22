use crate::user::user::AuthenticatedUser;

pub struct PlayerCharacter {
    pub name: String,
    pub user: AuthenticatedUser
}

impl PlayerCharacter {
    pub fn new(name: String, user: AuthenticatedUser) -> Self {
        PlayerCharacter { name, user }
    }

    pub fn hash(&self) -> String {
        let mut unique_id = String::new();
        unique_id.push_str(self.name.as_str());
        unique_id.push_str(self.user.name.as_str());

        return unique_id;
    }
}
