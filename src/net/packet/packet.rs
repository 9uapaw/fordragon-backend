use crate::error::error::Error;
use crate::net::protocol::encode::BBEncodable;
use crate::net::protocol::opcode::NetworkSendOpCode;
use bytes::{BufMut, Bytes, BytesMut};

pub struct S2CPacket<T> {
    opcode: NetworkSendOpCode,
    data: Option<T>,
}

pub struct S2CPacketBuilder<T: BBEncodable> {
    opcode: Option<NetworkSendOpCode>,
    data: Option<T>,
}

impl<T: BBEncodable> BBEncodable for S2CPacket<T> {
    fn encode_as_bbp(&self, buf: &mut BytesMut) {
        buf.put_u8(self.opcode as u8);
        if let Some(data) = &self.data {
            data.encode_as_bbp(buf);
        }
    }
}

impl<T: BBEncodable> S2CPacketBuilder<T> {
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

    pub fn data(&mut self, data: T) -> &mut Self {
        self.data.replace(data);
        self
    }

    pub fn build(&mut self) -> Result<S2CPacket<T>, Error> {
        Ok(S2CPacket {
            opcode: self
                .opcode
                .take()
                .ok_or(Error::new_network("No opcode was given in S2CPacket"))?,
            data: self.data.take(),
        })
    }
}
