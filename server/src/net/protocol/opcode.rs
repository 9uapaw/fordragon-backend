use crate::net::protocol::encode::BBEncodable;
use bytes::{Bytes, BytesMut, BufMut};
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
        buf.put_u16_le(*self as u16);
    }
}


/// Server to Client (Send) protocol package types, represented as two bytes
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum NetworkSendOpCode {
    UNKNOWN,
    AUTH,
    PLAYER_STATE_CHANGE,
}

impl BBEncodable for NetworkSendOpCode {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        buf.put_u16_le(*self as u16)
    }
}
