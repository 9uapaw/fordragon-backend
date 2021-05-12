use std::fmt::{Display, Formatter};

#[derive(Debug)]
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

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}", match self {
                AuthError::INVALID_USER => "Invalid user or password",
            }
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Error::NetworkError(s) => format!("NetworkError: {}", s),
                Error::AuthError(e) => e.to_string(),
            }
        )
    }
}
