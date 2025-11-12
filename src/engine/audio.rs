use crate::synth::AdsrTransform;
use crate::synth::DelayTransform;
use crate::synth::SineSource;
use crate::synth::Source;
use crate::synth::Synthesizer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Audio {
    device: cpal::Device,
    config: cpal::StreamConfig,
    sample_rate: f32,
}

impl Audio {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config = device
            .default_output_config()
            .expect("no default output config");
        let sample_rate = config.sample_rate().0 as f32;

        Self {
            device,
            config: config.into(),
            sample_rate,
        }
    }

    /// Play the tone for a few seconds.
    pub fn start(&self) {
        let mut sine = SineSource::new(self.sample_rate,  0.01, 1.0, 0.0);
        let mut synthesizer = Synthesizer::new(self.sample_rate);
        synthesizer.add_transform(Box::new(AdsrTransform::new(
            self.sample_rate,
            0.01,
            0.1,
            0.4,
            0.4,
        )));
        synthesizer.add_transform(Box::new(DelayTransform::new(self.sample_rate, 0.05, 0.5, 0.5)));
        synthesizer.note_on();

        let note_off_sample = (self.sample_rate * 1.0) as u64;

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _| {
                    // Send note_off at exactly 1 second
                    if synthesizer.get_sample() >= note_off_sample {
                        synthesizer.note_off();
                    }

                    for frame in data.chunks_mut(2) {
                        let sample = synthesizer.next_sample();

                        // Write to each channel
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
