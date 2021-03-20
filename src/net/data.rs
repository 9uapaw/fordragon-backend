use num_enum::TryFromPrimitive;
use crate::net::protocol::opcode::NetworkRecvOpCode;

/// The intermediate representation of a BBP message.
#[derive(Debug)]
pub enum IntermediateGameData {
    Auth { user: String, hash: String },
    Flag { op_code: NetworkRecvOpCode },
    PlayerInput {user: String, action: PlayerInputAction}
}

impl Default for IntermediateGameData {
    fn default() -> Self {
        IntermediateGameData::Flag {
            op_code: NetworkRecvOpCode::UNKNOWN,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum PlayerInputAction {
    MoveForward,
    StopMove
}
