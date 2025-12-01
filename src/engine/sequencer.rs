use crate::engine::audio::*;
use crate::messaging::Action;
use crate::types::PatternId;
use crossbeam::channel::{Receiver, Sender, bounded};
use std::thread;

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
        let rx = self.rx.clone();
        let tx = self.tx.clone();

        thread::spawn(move || {
            while let Ok(action) = rx.recv() {
                match action {
                    Action::PlayPhrase(phrase_id) => {
                        let (reply_tx, reply_rx) = bounded(1); // one-shot channel
                        tx.send(Action::GetPhraseData {
                            phrase_id,
                            reply_to: reply_tx,
                        })
                        .unwrap();

                        let phrase_data = reply_rx.recv().unwrap();
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn tick(&self) {}
}
