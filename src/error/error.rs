pub enum Error {
    NetworkError(String),
    AuthError(AuthError),
}

impl Error {
    pub fn new_network(msg: &str) -> Self {
        Error::NetworkError(msg.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AuthError {
    INVALID_USER,
}

impl AuthError {
    pub fn invalid_user_or_password() -> Error {
        Error::AuthError(AuthError::INVALID_USER)
    }
}
