use crate::game::location::pos::Position;
use crate::net::protocol::encode::{BBEncodable, ByteEncoder};
use bytes::BytesMut;

pub struct SpawnPacket {
    obj_id: String,
    location: Position
}

impl SpawnPacket {
    pub fn new(obj_id: String, location: Position) -> Self {
        SpawnPacket { obj_id, location }
    }
}

impl BBEncodable for SpawnPacket {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
       let mut encoder = ByteEncoder::new(buf);
        encoder.encode(&self.location);
    }
}