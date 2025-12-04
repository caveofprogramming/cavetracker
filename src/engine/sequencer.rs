use crate::engine::audio::*;
use crate::messaging::Action;
use crate::types::PatternId;
use crossbeam::channel::{Receiver, Sender, unbounded, bounded};
use std::thread;

pub struct Sequencer {
    tx: Sender<Action>,
    rx: Receiver<Action>,
    bpm: f32,
    sample_rate: u64,
    playing: bool,
    phrase_tx: Sender<Action>,
    phrase_rx: Receiver<Action>,
}

impl Sequencer {

    
    pub fn new(tx: Sender<Action>, rx: Receiver<Action>, sample_rate: u64, bpm: f32) -> Self {
        
        let (phrase_tx, phrase_rx):(Sender<Action>, Receiver<Action>) = unbounded();
        
        Self {
            sample_rate,
            bpm,
            playing: false,
            tx,
            rx,
            phrase_tx,
            phrase_rx,
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
