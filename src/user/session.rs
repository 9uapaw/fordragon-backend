use sha2::Digest;
use std::collections::HashMap;

const TEMP_SECRET: &str = "secret12345";

pub trait UserSessionManager {
    fn is_auth_registered(&self, user: &str, auth: &str) -> bool;
}

pub struct DefaultSessionManager {
    temp_storage: HashMap<String, String>,
}

impl DefaultSessionManager {
    pub fn new() -> Self {
        let mut temp_storage = HashMap::new();
        temp_storage.insert("admin".to_string(), format!("admin:admin:{}", TEMP_SECRET));
        DefaultSessionManager {
            temp_storage,
        }
    }
}

impl UserSessionManager for DefaultSessionManager {
    fn is_auth_registered(&self, user: &str, auth: &str) -> bool {
        if let Some(hash) = self.temp_storage.get(user) {
            debug!("Stored hash {} ?= {}", hash, auth);
            hash == auth
        } else {
            false
        }
    }
}
