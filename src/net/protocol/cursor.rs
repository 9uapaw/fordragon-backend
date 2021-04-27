use std::slice::Iter;
use bytes::BytesMut;

/// Isolates the progress on byte processing.
///
/// It has an internal byte array index pointer to the latest, successful conversion.
pub struct ByteCursor<'a> {
    current: usize,
    buf: Iter<'a, u8>,
}

impl<'a> ByteCursor<'a> {
    pub fn new(buf: &'a BytesMut) -> Self {
        ByteCursor {
            buf: buf.iter(),
            current: 0,
        }
    }

    /// Converts a byte to an u8 value.
    ///
    /// # Returns
    /// None if the conversion is unsuccessful due to missing bytes in the array.
    pub fn as_u8(&mut self) -> Option<u8> {
        let a = self.buf.next();

        match a {
            Some(a) => {
                self.current += 1;
                Some(u8::from_le_bytes([*a]))
            }
            _ => None,
        }
    }

    /// Converts two bytes to an u16 value in a little endian fashion.
    ///
    /// # Returns
    /// None if the conversion is unsuccessful due to missing bytes in the array.
    pub fn as_u16(&mut self) -> Option<u16> {
        let a = self.buf.next();
        let b = self.buf.next();

        match (a, b) {
            (Some(a), Some(b)) => {
                self.current += 2;
                Some(u16::from_le_bytes([*a, *b]))
            }
            _ => None,
        }
    }

    /// Converts four bytes to an u32 value in a little endian fashion.
    ///
    /// # Returns
    /// None if the conversion is unsuccessful due to missing bytes in the array.
    pub fn as_u32(&mut self) -> Option<u32> {
        let a = self.buf.next();
        let b = self.buf.next();
        let c = self.buf.next();
        let d = self.buf.next();

        match (a, b, c, d) {
            (Some(a), Some(b), Some(c), Some(d)) => {
                self.current += 4;
                Some(u32::from_le_bytes([*a, *b, *c, *d]))
            }
            _ => None,
        }
    }

    pub fn as_utf8(&mut self) -> Option<String> {
        let len = self.as_u32();
        match len {
            Some(len) => {
                let mut bytes = Vec::new();
                for i in 0..len {
                   if let Some(b) = self.buf.next() {
                       bytes.push(*b);
                   } else {
                       return None
                   }
                }
                return String::from_utf8(bytes).ok()
            },
            _ => None
        }
    }

}
#[cfg(test)]
mod tests {
    use crate::net::protocol::decode::{ByteToRawDecoder};
    use crate::net::protocol::cursor::{ByteCursor};
    use bytes::BytesMut;
    use crate::net::protocol::opcode::NetworkRecvOpCode;
    use crate::net::data::IntermediateGamePacket;

    #[test]
    fn test_u16() {
        let mut vec_bytes = Vec::new();
        vec_bytes.extend_from_slice(&300u16.to_le_bytes());
        vec_bytes.extend_from_slice(&400u16.to_le_bytes());

        let mut bytes = BytesMut::from(vec_bytes.as_slice());

        let mut cursor = ByteCursor::new(&bytes);
        let first = cursor.as_u16();
        let second = cursor.as_u16();
        let third = cursor.as_u16();

        match (first, second, third) {
            (Some(a), Some(b), None) => {
                assert_eq!(a, 300);
                assert_eq!(b, 400);
            }
            _ => panic!("Only 4 bytes were added"),
        }
    }

    #[test]
    fn test_u32() {
        let mut vec_bytes = Vec::new();
        vec_bytes.extend_from_slice(&300u32.to_le_bytes());
        vec_bytes.extend_from_slice(&400u32.to_le_bytes());

        let mut bytes = BytesMut::from(vec_bytes.as_slice());

        let mut cursor = ByteCursor::new(&bytes);
        let first = cursor.as_u16();
        let second = cursor.as_u16();
        let third = cursor.as_u16();

        match (first, second, third) {
            (Some(a), Some(b), None) => {
                assert_eq!(a, 300);
                assert_eq!(b, 400);
            }
            _ => panic!("Parsing failed"),
        }
    }
    #[test]
    fn test_utf8_str() {
        let mut vec_bytes = Vec::new();
        vec_bytes.extend_from_slice(&("test".bytes().len() as u32).to_le_bytes());
        vec_bytes.extend_from_slice("test".as_bytes());
        vec_bytes.extend_from_slice(&("test2".bytes().len() as u32).to_le_bytes());
        vec_bytes.extend_from_slice("test2".as_bytes());

        let mut bytes = BytesMut::from(vec_bytes.as_slice());

        let mut cursor = ByteCursor::new(&bytes);
        let first = cursor.as_utf8();
        let second = cursor.as_utf8();
        let third = cursor.as_utf8();

        match (first, second, third) {
            (Some(a), Some(b), None) => {
                assert_eq!(a, "test");
                assert_eq!(b, "test2");
            }
            _ => panic!("Parsing failed"),
        }
    }
}
