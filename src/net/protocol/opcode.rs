use crate::net::protocol::encode::BBEncodable;
use bytes::Bytes;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum NetworkRecvOpCode {
    UNKNOWN,
    AUTH,
}

impl Default for NetworkRecvOpCode {
    fn default() -> Self {
        NetworkRecvOpCode::UNKNOWN
    }
}

impl BBEncodable for NetworkRecvOpCode {
    fn encode_as_bbp(&self) -> Bytes {
        let bytes = (*self as u16).to_le_bytes();
        Bytes::from(Vec::from(bytes))
    }
}
