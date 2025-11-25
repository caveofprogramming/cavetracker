use crate::engine::Sequencer;
use crate::engine::audio::*;
use crate::messaging::Action;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::Sender;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Audio {
    device: cpal::Device,
    config: cpal::StreamConfig,
    sample_rate: u64,
    stream: Option<cpal::Stream>,
    tx: Sender<Action>,
    instrument_manager: Arc<Mutex<InstrumentManager>>,
}

impl Audio {
    pub fn new(instrument_manager: Arc<Mutex<InstrumentManager>>, tx: Sender<Action>) -> Self {
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
            instrument_manager,
        }
    }

    pub fn get_sample_rate(&self) -> u64 {
        self.sample_rate
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }

    pub fn start(&mut self) {
        let instrument_manager = self.instrument_manager.clone();
        let sample_rate = self.sample_rate as f32;

        instrument_manager.lock().note_on();

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _| {
                    for frame in data.chunks_mut(2) {
                        let sample = instrument_manager.lock().next();
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
