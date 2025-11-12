use crate::engine::Source;

pub struct SineSource {
    phase: f32,
    freq: f32,
    sample_rate: u64,
}

impl SineSource {
    pub fn new(sample_rate: u64, freq: f32) -> Self {
        Self {
            freq,
            sample_rate,
            phase: 0.0,
        }
    }
}

impl Source for SineSource {
    fn next_sample(&mut self) -> f32 {
        let value = (2.0 * std::f32::consts::PI * self.phase).sin();
        self.phase = (self.phase + self.freq / (self.sample_rate as f32)) % 1.0;
        value
    }

    fn param_names(&self) -> Vec<String> {
        ["freq", "phase"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn set_param(&mut self, name: &str, value: f32) {
        match name {
            "freq" => self.freq = value,
            "phase" => self.phase = value,
            _ => (),
        }
    }
}
