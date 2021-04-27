use num_enum::TryFromPrimitive;
use crate::net::protocol::opcode::NetworkRecvOpCode;

/// The intermediate representation of a BBP C2S message.
#[derive(Debug)]
pub enum IntermediateGamePacket {
    Auth { user: String, hash: String },
    Flag { op_code: NetworkRecvOpCode },
    PlayerInput {user: String, action: PlayerInputAction}
}

impl Default for IntermediateGamePacket {
    fn default() -> Self {
        IntermediateGamePacket::Flag {
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
