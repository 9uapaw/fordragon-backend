use crate::net::provider::{DataStreamReader, DataStreamWriter};
use std::net::SocketAddr;
use std::fmt::{Display, Formatter};

pub struct AuthenticatedUser {
    pub addr: SocketAddr,
    pub name: String,
    reader: Option<DataStreamReader>,
    writer: Option<DataStreamWriter>,
}

impl AuthenticatedUser {
    pub fn new(
        addr: SocketAddr,
        name: String,
        reader: Option<DataStreamReader>,
        writer: Option<DataStreamWriter>,
    ) -> Self {
        AuthenticatedUser {
            addr,
            name,
            reader,
            writer,
        }
    }
}

impl Display for AuthenticatedUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.addr, &self.name)
    }
}
