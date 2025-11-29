use crate::engine::audio::*;
use crate::messaging::Action;
use crate::types::PatternId;
use crossbeam::channel::{Receiver, Sender};

pub struct Sequencer {
    tx: Sender<Action>,
    rx: Receiver<Action>,
    bpm: f32,
    sample_rate: u64,
    playing: bool,
}

impl Sequencer {
    pub fn new(tx: Sender<Action>, rx: Receiver<Action>, sample_rate: u64, bpm: f32) -> Self {
        Self {
            sample_rate,
            bpm,
            playing: false,
            tx,
            rx,
        }
    }

    pub fn run(&self) {

    }

    pub fn tick(&self) {}
}
