pub struct CanMessage {
    pub id: u16,
    pub data: u16,
}

pub trait CanInput {
    fn receive(&mut self) -> Result<CanMessage, ()>;
}
