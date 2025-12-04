use crate::engine::Sequencer;
use crate::engine::audio::*;
use crate::messaging::Action;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::Sender;
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::DerefMut;

pub struct Audio {
    device: cpal::Device,
    config: cpal::StreamConfig,
    sample_rate: u64,
    stream: Option<cpal::Stream>,
    tx: Sender<Action>,
    instrument_manager: Arc<Mutex<InstrumentManager>>,
    sequencer: Arc<Mutex<Sequencer>>,
    is_playing: Arc<AtomicBool>,
}

impl Audio {
    pub fn new(
        tx: Sender<Action>,
        instrument_manager: Arc<Mutex<InstrumentManager>>,
        sequencer: Arc<Mutex<Sequencer>>,
    ) -> Self {
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
            sequencer,
            is_playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Acquire)
    }

    pub fn get_sample_rate(&self) -> u64 {
        self.sample_rate
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            self.is_playing.store(false, Ordering::Release);
            drop(stream);
        }
    }

    pub fn start(&mut self) {
        let instrument_manager = self.instrument_manager.clone();
        let sequencer = self.sequencer.clone();
        let sample_rate = self.sample_rate as f32;

        let is_playing = Arc::clone(&self.is_playing);
        let is_playing_error = Arc::clone(&self.is_playing);

        instrument_manager.lock().note_on();

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _| {
                    for frame in data.chunks_mut(2) {
                        is_playing.store(true, Ordering::Release);
                        sequencer.lock().deref_mut().tick();
                        let sample = instrument_manager.lock().next();
                        frame[0] = sample;
                        frame[1] = 0.2 * sample;
                    }
                },
                move |err| {
                    is_playing_error.store(false, Ordering::Release);
                    eprintln!("audio error: {err}");
                },
                None,
            )
            .expect("failed to build output stream");

        self.is_playing.store(true, Ordering::Release);

        stream.play().expect("failed to start stream");

        self.stream = Some(stream);
    }
}
