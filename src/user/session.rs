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
        let mut temp_hash = sha2::Sha256::new();
        temp_hash.update(&format!("admin:admin:{}", TEMP_SECRET).as_bytes());
        temp_storage.insert("admin", format!("{:x}", &temp_hash.finalize()));
        println!("{}", temp_storage.get("admin").unwrap());
        DefaultSessionManager {
            temp_storage: HashMap::new(),
        }
    }
}

impl UserSessionManager for DefaultSessionManager {
    fn is_auth_registered(&self, user: &str, auth: &str) -> bool {
        if let Some(hash) = self.temp_storage.get(user) {
            hash == auth
        } else {
            false
        }
    }
}
