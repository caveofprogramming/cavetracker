use crate::engine::audio::*;
use crossbeam::channel::Receiver;

pub struct InstrumentManager {
    sample_rate: f32,
    instruments: Vec<Instrument>,
}

impl InstrumentManager {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.00,
            instruments: vec![],
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn add_synth(&mut self) {
        let patch = Patch {
            sample_rate: self.sample_rate,
            nodes: vec![
                NodeDef::Sine(SineDef {}),
                NodeDef::Lfo(LfoDef {
                    freq: 0.2,
                    depth: 50.0,
                    offset: 0.0,
                    target_node: 0,
                    target_param: param::FREQUENCY,
                }),
                NodeDef::Adsr(AdsrDef {
                    attack: 0.01,
                    decay: 0.4,
                    sustain: 0.0,
                    release: 1.0,
                    target_node: 0,
                    target_param: param::AMPLITUDE,
                }),
            ],
            connections: vec![],
        };

        let instrument = Instrument::from_patch(&patch);
        self.instruments.push(instrument);
    }

    pub fn next(&mut self) -> f32 {
        let mut mix = 0.0;
        for instrument in &mut self.instruments {
            mix += instrument.next();
        }

        let count = self.instruments.len();
        if count != 0 {
            mix /= count as f32;
        }

        mix
    }
}
