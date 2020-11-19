use crate::net::protocol::opcode::NetworkRecvOpCode;

#[derive(Debug)]
pub enum RawInternalData {
    AUTH { user: String, hash: String },
    FLAG { op_code: NetworkRecvOpCode },
}

impl Default for RawInternalData {
    fn default() -> Self {
        RawInternalData::FLAG {
            op_code: NetworkRecvOpCode::UNKNOWN,
        }
    }
}
