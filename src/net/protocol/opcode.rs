use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
