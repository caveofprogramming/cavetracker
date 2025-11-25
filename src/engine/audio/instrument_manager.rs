use crate::engine::audio::*;
use crossbeam::channel::Receiver;

pub struct InstrumentManager {
    sample_rate: f32,
    instruments: Vec<Instrument>,
}

impl InstrumentManager {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            instruments: vec![],
        }
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
}
