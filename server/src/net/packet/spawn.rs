use crate::game::location::pos::Position;
use crate::net::protocol::encode::{BBEncodable, ByteEncoder};
use bytes::BytesMut;

#[derive(Debug)]
pub struct SpawnPacket {
    location: Position
}

impl SpawnPacket {
    pub fn new(location: Position) -> Self {
        SpawnPacket { location }
    }
}

impl BBEncodable for SpawnPacket {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
       let mut encoder = ByteEncoder::new(buf);
        encoder.encode(&self.location);
    }
}