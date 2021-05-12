use crate::net::protocol::encode::BBEncodable;
use bytes::{Bytes, BytesMut, BufMut};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use crate::error::error::AuthError;
use crate::net::protocol::opcode::NetworkSendOpCode;

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum AuthPackage {
    AUTH_OK,
    AUTH_INVALID_CRED,
    AUTH_NETWORK_ERR
}

impl From<&AuthError> for AuthPackage {
    fn from(e: &AuthError) -> Self {
        match e {
           AuthError::INVALID_USER => AuthPackage::AUTH_INVALID_CRED
        }
    }
}

impl BBEncodable for AuthPackage {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        NetworkSendOpCode::AUTH.encode_as_bbp(buf);
        buf.put_u16_le(*self as u16);
    }
}