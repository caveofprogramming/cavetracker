

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::Sender;
use crate::messaging::Action;
use crate::engine::audio::*;
use crate::engine::Sequencer;

pub struct Audio {
    device: cpal::Device,
    config: cpal::StreamConfig,
    sample_rate: u64,
    stream: Option<cpal::Stream>,
    tx: Sender<Action>,
}

impl Audio {
    pub fn new(tx: Sender<Action>) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config = device
            .default_output_config()
            .expect("no default output config");
        let sample_rate = config.sample_rate().0 as u64;

        Self {
            device,
            config: config.into(),
            sample_rate,
            stream: None,
            tx,
        }
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }

    pub fn start(&mut self) {
        let sample_rate = self.sample_rate as f32;

        let patch = Patch {
            sample_rate: sample_rate,
            nodes: vec![
                NodeDef::Sine(SineDef {}),
                /*NodeDef::Lfo(LfoDef {
                    freq: 0.2,
                    depth: 50.0,
                    offset: 0.0,
                    target_node: 0,
                    target_param: param::FREQUENCY,
                }),*/
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

        let mut synth = Synth::new(patch, 32);
        // Play a C major chord
        //synth.note_on(60, 127); // C
        //synth.note_on(64, 127); // E
        synth.note_on(70, 127); // G
        synth.note_on(77, 127); // G

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _| {
                    for frame in data.chunks_mut(2) {
                        let sample = synth.next_sample();
                        frame[0] = sample;
                        frame[1] = 0.2 * sample;
                    }
                },
                |err| eprintln!("audio error: {err}"),
                None,
            )
            .expect("failed to build output stream");

        stream.play().expect("failed to start stream");

        self.stream = Some(stream);
    }
}
