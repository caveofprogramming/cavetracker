use crate::messaging::Action;
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::thread;

pub struct Dispatcher {
    rx: Receiver<Action>,
    update_tx: Sender<Action>,
    sequencer_tx: Sender<Action>,
    audio_tx: Sender<Action>,
    is_playing: bool,
}

impl Dispatcher {
    pub fn new(
        rx: Receiver<Action>,
        update_tx: Sender<Action>,
        sequencer_tx: Sender<Action>,
        audio_tx: Sender<Action>,
    ) -> Self {
        Self {
            rx,
            update_tx,
            sequencer_tx,
            audio_tx,
            is_playing: false,
        }
    }

    pub fn run(&self) {
        let sequencer_tx = self.sequencer_tx.clone();
        let audio_tx = self.audio_tx.clone();
        let update_tx = self.update_tx.clone();
        let rx = self.rx.clone();

        thread::spawn(move || {
            while let Ok(action) = rx.recv() {
                match &action {
                    Action::PlayPhrase(_) => {
                        sequencer_tx.send(action.clone()).unwrap();
                        audio_tx.send(action.clone()).unwrap();
                    }
                    _ => {
                        update_tx.send(action.clone()).unwrap();
                    }
                }
            }
        });
    }
}
