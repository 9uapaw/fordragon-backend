use bytes::Bytes;

// A BombShell Protocol compliant encoder, that
pub struct ByteEncoder {}

impl ByteEncoder {
    pub fn new() -> Self {
        ByteEncoder {}
    }

    pub fn encode_str(&self, s: &str) -> Bytes {
        let len = s.bytes().len() as u32;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&len.to_le_bytes());
        bytes.extend_from_slice(s.as_bytes());

        Bytes::from(bytes)
    }
}
