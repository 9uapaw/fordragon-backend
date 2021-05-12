use std::time::{Duration, Instant};

pub struct FrameResource {
    pub frame_delta: Duration,
}

impl Default for FrameResource {
    fn default() -> Self {
        FrameResource {
            frame_delta: Duration::new(0, 0)
        }
    }
}
