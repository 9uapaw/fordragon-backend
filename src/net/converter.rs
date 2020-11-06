use crate::net::data::RawInternalData;
use bytes::BytesMut;

pub struct ByteToRawConverter {}

impl ByteToRawConverter {
    pub fn new() -> Self {
        ByteToRawConverter {}
    }

    pub fn convert(&self, buf: &BytesMut) -> RawInternalData {
        RawInternalData::new()
    }
}