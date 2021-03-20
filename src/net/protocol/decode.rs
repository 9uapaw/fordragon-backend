use crate::error::error::Error;
use crate::net::data::{IntermediateGameData, PlayerInputAction};
use crate::net::protocol::cursor::ByteCursor;
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
    pub fn convert(&self, buf: &BytesMut) -> Result<IntermediateGameData, Error> {
        let mut cursor = ByteCursor::new(buf);
        let mut data = IntermediateGameData::default();

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

    #[inline]
    fn convert_by_op(
        &self,
        cursor: &mut ByteCursor,
        op_code: &NetworkRecvOpCode,
    ) -> Result<IntermediateGameData, Error> {
        match op_code {
            NetworkRecvOpCode::AUTH => convert_auth(cursor),
            NetworkRecvOpCode::MOVEMENT => convert_movement(cursor),
            NetworkRecvOpCode::UNKNOWN => Err(Error::new_network("Invalid OpCode")),
        }
    }
}

#[inline]
fn convert_auth(cursor: &mut ByteCursor) -> Result<IntermediateGameData, Error> {
    let user = cursor
        .as_utf8()
        .ok_or(Error::new_network("Invalid or missing username from AUTH"))?;
    let hash = cursor
        .as_utf8()
        .ok_or(Error::new_network("Invalid or missing hash from AUTH"))?;
    Ok(IntermediateGameData::Auth { user, hash })
}

#[inline]
fn convert_movement(cursor: &mut ByteCursor) -> Result<IntermediateGameData, Error> {
    let user = cursor.as_utf8().ok_or(Error::new_network(
        "Invalid or missing username from InputPacket",
    ))?;
    let action = cursor.as_u8().ok_or(Error::new_network(
        "Invalid or missing action type from InputPacket",
    ))?;
    Ok(IntermediateGameData::PlayerInput {
        user,
        action: PlayerInputAction::try_from(action)
            .map_err(|e| Error::NetworkError(e.to_string()))?,
    })
}

#[cfg(test)]
mod tests {
    use crate::net::data::IntermediateGameData;
    use crate::net::protocol::cursor::ByteCursor;
    use crate::net::protocol::decode::ByteToRawDecoder;
    use crate::net::protocol::encode::ByteEncoder;
    use crate::net::protocol::opcode::NetworkRecvOpCode;
    use bytes::{Buf, BytesMut};

    #[test]
    fn test_auth() {
        let converter = ByteToRawDecoder::new();
        let mut bytes = BytesMut::new();
        let mut encoder = ByteEncoder::new(&mut bytes);
        encoder.encode(&NetworkRecvOpCode::AUTH);
        encoder.encode_str("test_user");
        encoder.encode_str("hash12345");

        let raw = converter.convert(&bytes);

        if let Ok(raw) = raw {
            match raw {
                IntermediateGameData::Auth { user, hash } => {
                    assert_eq!(user, "test_user");
                    assert_eq!(hash, "hash12345");
                }
                _ => panic!("Invalid network opcode"),
            }
        } else {
            panic!("Unsuccessful conversion")
        }
    }
}
