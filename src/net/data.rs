use crate::net::protocol::opcode::NetworkRecvOpCode;

/// The intermediate representation of a BBP message.
#[derive(Debug)]
pub enum IntermediateGameData {
    Auth { user: String, hash: String },
    Flag { op_code: NetworkRecvOpCode },
}

impl Default for IntermediateGameData {
    fn default() -> Self {
        IntermediateGameData::Flag {
            op_code: NetworkRecvOpCode::UNKNOWN,
        }
    }
}
