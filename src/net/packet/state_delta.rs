use crate::game::location::pos::Position;
use legion::Entity;
use crate::net::packet::spawn::SpawnPacket;
use crate::common::obj_id::GameObjectIdentifier;
use crate::net::protocol::encode::{BBEncodable, ByteEncoder};
use bytes::{BytesMut, BufMut};

pub struct ObjectStateDeltaPacket {
    pub id: GameObjectIdentifier,
    pub delta_batch: ObjectStateBatch
}

impl ObjectStateDeltaPacket {
    pub fn new(id: GameObjectIdentifier, delta_batch: ObjectStateBatch) -> Self {
        ObjectStateDeltaPacket { id, delta_batch }
    }
}

impl BBEncodable for ObjectStateDeltaPacket {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        let mut encoder = ByteEncoder::new(buf);
        encoder.encode_str(self.id.external.as_str());
        encoder.encode(&self.delta_batch)
    }
}

pub struct ObjectStateBatch {
    pub batch: Vec<ObjectStateChange>
}

impl ObjectStateBatch {
    pub fn new() -> Self {
        ObjectStateBatch { batch: Vec::new() }
    }

    pub fn add(&mut self, state_change: ObjectStateChange) {
        self.batch.push(state_change);
    }
}

impl BBEncodable for ObjectStateBatch {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        let mut bytes = BytesMut::new();
        for state in &self.batch {
            state.encode_as_bbp(&mut bytes);
        }
        buf.put_u32_le(bytes.len() as u32);
        buf.extend(bytes);
    }
}

pub enum ObjectStateChange {
    Position(Position),
    Speed(f32),
    Spawn(SpawnPacket)
}

impl BBEncodable for ObjectStateChange {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        match self {
            ObjectStateChange::Position(p) => {
                buf.put_u8(0);
                p.encode_as_bbp(buf);
            },
            ObjectStateChange::Speed(s) => {
                buf.put_u8(1);
                buf.put_f32_le(*s);
            },
            ObjectStateChange::Spawn(sp) => {
                buf.put_u8(2);
                sp.encode_as_bbp(buf);
            }
        }
    }
}
