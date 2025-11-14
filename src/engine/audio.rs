use crate::engine::{SineSource, Source, Instrument};
use crate::messaging::Action;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::Sender;
use std::f32::consts::PI;

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
        let mut sine = SineSource::new(self.sample_rate, 440.0);
        let mut instrument = Instrument::new();
        instrument.add_source(Box::new(sine));

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _| {
                    for frame in data.chunks_mut(2) {
                        let sample = instrument.next_sample(0.0);
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
