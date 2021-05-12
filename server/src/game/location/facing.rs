use euclid::{Angle, UnknownUnit};

const MAX_FACING: f32 = std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Facing {
    angle: Angle<f32>,
}

impl Facing {
    pub fn new() -> Self {
        Facing {
            angle: Angle::radians(0.0),
        }
    }

    pub fn rad(rad: f32) -> Self {
        Facing {
            angle: Angle::radians(rad),
        }
    }

    pub fn get_facing(&self) -> f32 {
        self.angle.get()
    }
}
