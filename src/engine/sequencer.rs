use crate::engine::audio::*;
use crate::types::PatternId;

pub struct Sequencer {
    bpm: f32,
    sample_rate: u64,
}

impl Sequencer {
    pub fn new(sample_rate: u64, bpm: f32) -> Self {
        Self { sample_rate, bpm }
    }

    pub fn tick(&self, synth: &mut Synth) {}
}
