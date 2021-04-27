use bytes::{Bytes, BytesMut, BufMut};

pub trait BBEncodable {
    fn encode_as_bbp(&self, buf: &mut BytesMut);
}

///! A BombShell Protocol compliant encoder
pub struct ByteEncoder<'a> {
    buf: &'a mut BytesMut,
}

impl<'a> ByteEncoder<'a> {
    pub fn new(buf: &'a mut BytesMut) -> Self {
        ByteEncoder { buf }
    }

    pub fn encode_str(&mut self, s: &str) {
        let len = s.bytes().len() as u32;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&len.to_le_bytes());
        bytes.extend_from_slice(s.as_bytes());
        self.buf.extend(bytes);
    }

    pub fn encode_u16(&mut self, v: u16) {
        self.buf.put_u16_le(v);
    }

    pub fn encode_u32(&mut self, v: u32) {
        self.buf.put_u32_le(v);
    }

    pub fn encode_u8(&mut self, v: u8) {
        self.buf.put_u8(v);
    }

    pub fn encode<T: BBEncodable>(&mut self, c: &T) {
        c.encode_as_bbp(self.buf);
    }

    pub fn buf(&self) -> &[u8] {
        self.buf.as_ref()
    }
}
