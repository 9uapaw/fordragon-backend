use crate::net::protocol::encode::BBEncodable;
use bytes::{Bytes, BytesMut};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

/// Client to Server (Recv) protocol package types, represented as two bytes
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum NetworkRecvOpCode {
    UNKNOWN,
    AUTH,
    MOVEMENT
}

impl Default for NetworkRecvOpCode {
    fn default() -> Self {
        NetworkRecvOpCode::UNKNOWN
    }
}

impl BBEncodable for NetworkRecvOpCode {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        let bytes = (*self as u16).to_le_bytes();
        buf.extend(bytes.iter());
    }
}


/// Server to Client (Send) protocol package types, represented as two bytes
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum NetworkSendOpCode {
    UNKNOWN,
    AUTH,
    SPAWN,
}

impl BBEncodable for NetworkSendOpCode {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        let bytes = (*self as u16).to_le_bytes();
        buf.extend(bytes.iter());
    }
}
