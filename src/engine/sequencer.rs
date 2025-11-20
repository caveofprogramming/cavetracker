use crate::types::PatternId;
use crate::engine::audio::*;


pub struct Sequencer {
    bpm: f32,
    sample_rate: f32,
    samples_per_step: u64,
    current_sample: u64,
    current_pattern: PatternId,
    phrase: Vec<Option<(u8, u8)>>, // note, velocity
}

impl Sequencer {
    pub fn new(sample_rate: u64, bpm: f32) {

    }

    pub fn tick(&self, synth: &mut Synth) {

    }
}

