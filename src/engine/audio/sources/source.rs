pub trait Source: Send + Sync {
    fn next(&mut self) -> f32;
    fn get_param(&self, param: ParamId) -> f32;
    fn set_param(&mut self, param: ParamId, value: f32);
    fn set(&mut self, note: u8, velocity: u8);
}

pub type NodeId = usize;
pub type ParamId = u32;

pub mod param {
    pub const AMPLITUDE: u32 = 1000;
    pub const FREQUENCY: u32 = 1001;
}
