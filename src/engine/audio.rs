use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use crate::engine::{Source, SineSource};

pub struct Audio {
    device: cpal::Device,
    config: cpal::StreamConfig,
    sample_rate: u64,
}

impl Audio {
    pub fn new() -> Self {
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
        }
    }

    pub fn start(&self) {
        let mut sine = SineSource::new(self.sample_rate, 440.0);

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _| {
                    for frame in data.chunks_mut(2) {
                        let sample = sine.next_sample();
                        frame[0] = sample;
                        frame[1] = 0.2 * sample;
                    }
                },
                |err| eprintln!("audio error: {err}"),
                None,
            )
            .expect("failed to build output stream");

        stream.play().expect("failed to start stream");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
