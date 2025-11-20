use crate::engine::audio::{self, Modulator, NodeId, ParamId, Source};
use std::f32::consts::PI;

pub struct Lfo {
    sample_rate: f32,
    phase: f32,
    target_id: NodeId,
    param_id: ParamId,
    freq: f32,
    range: f32,
    offset: f32,
    base_freq: Option<f32>,
}

impl Lfo {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            target_id: 0,
            param_id: 0,
            freq: 0.0,
            range: 0.0,
            offset: 0.0,
            base_freq: None,
        }
    }

    pub fn configure(
        &mut self,
        target_id: NodeId,
        param_id: ParamId,
        freq: f32,
        range: f32,
        offset: f32,
    ) {
        self.target_id = target_id;
        self.param_id = param_id;
        self.freq = freq;
        self.range = range;
        self.offset = offset;
    }
}

impl Modulator for Lfo {
    fn tick(&mut self, sources: &mut Vec<Box<dyn Source>>) {
        let source = &mut sources[self.target_id];
        let param_value = source.get_param(self.param_id);
        let base_freq = *self.base_freq.get_or_insert(param_value);

        self.phase += 2.0 * PI * self.freq / self.sample_rate;

        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        let lfo_value = self.phase.sin() * self.range + self.offset;

        source.set_param(self.param_id, base_freq + lfo_value);
    }

    fn note_on(&mut self) {
        self.phase = 0.0;
        self.base_freq = None;
    }

    fn note_off(&mut self) {}
    fn is_active(&self) -> bool {
        true
    }
}
