use std::f32::consts::PI;

use crate::engine::audio::{ParamId, Source, param};

#[derive(Debug)]
pub struct Sine {
    amplitude: f32,
    freq: f32,
    phase: f32,
    sample_rate: f32,
}

impl Sine {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            amplitude: 1.0,
            freq: 440.0,
            phase: 0.0,
            sample_rate,
        }
    }
}

impl Source for Sine {
    fn next(&mut self) -> f32 {
        self.phase += 2.0 * PI * self.freq / self.sample_rate;

        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        self.amplitude * self.phase.sin()
    }

    fn set(&mut self, note: u8, velocity: u8) {
        // MIDI note to frequency
        self.freq = 440.0 * (2.0f32).powf((note as f32 - 69.0) / 12.0);

        let vel_norm = velocity as f32 / 127.0;
        self.amplitude = vel_norm;
    }

    fn get_param(&self, param: ParamId) -> f32 {
        match param {
            param::FREQUENCY => self.freq,
            param::AMPLITUDE => self.amplitude,
            _ => 0.0,
        }
    }

    fn set_param(&mut self, param: ParamId, value: f32) {
        match param {
            param::FREQUENCY => self.freq = value,
            param::AMPLITUDE => self.amplitude = value,
            _ => {}
        }
    }
}
