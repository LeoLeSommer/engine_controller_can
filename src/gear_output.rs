#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GearState {
    Forward,
    Neutral,
    Reverse,
    Stop,
}

pub struct GearsState {
    pub starboard: GearState,
    pub port: GearState,
}

impl Default for GearsState {
    fn default() -> Self {
        GearsState {
            starboard: GearState::Neutral,
            port: GearState::Neutral,
        }
    }
}

pub trait GearOutput {
    fn set_pin1(&mut self, value: bool) -> Result<(), ()>;
    fn set_pin2(&mut self, value: bool) -> Result<(), ()>;
    fn set_pin3(&mut self, value: bool) -> Result<(), ()>;
    fn set_pin4(&mut self, value: bool) -> Result<(), ()>;
}
