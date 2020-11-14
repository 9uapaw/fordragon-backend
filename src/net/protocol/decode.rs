use crate::error::error::Error;
use crate::net::protocol::cursor::ByteCursor;
use crate::net::data::RawInternalData;
use crate::net::protocol::opcode::NetworkRecvOpCode;
use bytes::BytesMut;
use std::convert::TryFrom;
use std::slice::Iter;

/// A byte to Rust raw unstructured type converter
pub struct ByteToRawDecoder {}

impl ByteToRawDecoder {
    pub fn new() -> Self {
        ByteToRawDecoder {}
    }

    /// Converts raw bytes to unstructured Rust type
    ///
    /// The bytes are interpreted in a little endian fashion as the following:
    /// 0-2 bytes: NetworkOpCode (u16)
    pub fn convert(&self, buf: &BytesMut) -> Result<RawInternalData, Error> {
        let mut cursor = ByteCursor::new(buf);
        let mut data = RawInternalData::default();

        let op_code = match cursor.as_u16() {
            Some(op) => Some(
                NetworkRecvOpCode::try_from(op)
                    .map_err(|e| Error::NetworkError(format!("Invalid network op code {}", op)))?,
            ),
            None => None,
        };
        if let Some(op) = op_code {
            self.convert_by_op(&mut cursor, &op)
        } else {
            Err(Error::new_network(
                "Malformed protocol packet (missing op code)",
            ))
        }
    }

    fn convert_by_op(
        &self,
        cursor: &mut ByteCursor,
        op_code: &NetworkRecvOpCode,
    ) -> Result<RawInternalData, Error> {
        match op_code {
            NetworkRecvOpCode::AUTH => {
                let user = cursor
                    .as_utf8()
                    .ok_or(Error::new_network("Invalid or missing username from AUTH"))?;
                let hash = cursor
                    .as_utf8()
                    .ok_or(Error::new_network("Invalid or missing hash from AUTH"))?;
                Ok(RawInternalData::AUTH { user, hash })
            }
            _ => Err(Error::new_network("Invalid OpCode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::net::protocol::decode::ByteToRawDecoder;
    use crate::net::protocol::cursor::ByteCursor;
    use crate::net::data::RawInternalData;
    use crate::net::protocol::opcode::NetworkRecvOpCode;
    use bytes::{BytesMut, Buf};
    use crate::net::protocol::encode::ByteEncoder;

    #[test]
    fn test_auth() {
        let converter = ByteToRawDecoder::new();
        let encoder = ByteEncoder::new();

        let mut vec_bytes = Vec::new();
        vec_bytes.extend_from_slice(&(NetworkRecvOpCode::AUTH as u16).to_le_bytes());
        vec_bytes.extend_from_slice(encoder.encode_str("test_user").bytes());
        vec_bytes.extend_from_slice(encoder.encode_str("hash12345").bytes());

        let mut bytes = BytesMut::from(vec_bytes.as_slice());
        let raw = converter.convert(&bytes);

        if let Ok(raw) = raw {
            match raw {
                RawInternalData::AUTH { user, hash } => {
                    assert_eq!(user, "test_user");
                    assert_eq!(hash, "hash12345");
                },
                _ => panic!("Invalid network opcode"),
            }
        } else {
            panic!("Unsuccessful conversion")
        }
    }
}
