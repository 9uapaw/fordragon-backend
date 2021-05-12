use crate::error::error::Error;
use crate::net::protocol::encode::BBEncodable;
use crate::net::protocol::opcode::NetworkSendOpCode;
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::Debug;

pub struct S2CPacket<'a, T> {
    opcode: NetworkSendOpCode,
    data: Option<&'a T>,
}

pub struct S2CPacketBuilder<'a, T: BBEncodable> {
    opcode: Option<NetworkSendOpCode>,
    data: Option<&'a T>,
}

impl<'a, T: BBEncodable> BBEncodable for S2CPacket<'a, T> {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        self.opcode.encode_as_bbp(buf);
        if let Some(data) = &self.data {
            data.encode_as_bbp(buf);
        }
    }
}

impl<'a, T: BBEncodable> S2CPacketBuilder<'a, T> {
    pub fn new() -> Self {
        S2CPacketBuilder {
            opcode: None,
            data: None,
        }
    }

    pub fn op_code(&mut self, op_code: NetworkSendOpCode) -> &mut Self {
        self.opcode.replace(op_code);
        self
    }

    pub fn data(&mut self, data: &'a T) -> &mut Self {
        self.data.replace(data);
        self
    }

    pub fn build(&mut self) -> Result<S2CPacket<'a, T>, Error> {
        Ok(S2CPacket {
            opcode: self
                .opcode
                .take()
                .ok_or(Error::new_network("No opcode was given in S2CPacket"))?,
            data: self.data.take(),
        })
    }
}
