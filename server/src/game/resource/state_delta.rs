use crate::game::location::pos::Position;
use std::collections::VecDeque;
use crate::net::packet::state_delta::ObjectStateDeltaPacket;

pub struct StateDeltaCache(pub VecDeque<ObjectStateDeltaPacket>);

impl StateDeltaCache {
    pub fn new() -> Self {
        StateDeltaCache(VecDeque::new())
    }

}