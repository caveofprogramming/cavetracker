use crate::messaging::Action;
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::thread;

pub struct Dispatcher {
    rx: Receiver<Action>,
    update_tx: Sender<Action>,
    audio_tx: Sender<Action>,
}

impl Dispatcher {
    pub fn new(rx: Receiver<Action>, update_tx: Sender<Action>, audio_tx: Sender<Action>) -> Self {
        Self {
            rx,
            update_tx,
            audio_tx,
        }
    }

    pub fn run(self) {
        thread::spawn(move || {
            while let Ok(action) = self.rx.recv() {
                match &action {
                    // audio‑related actions
                    Action::TogglePlayPhrase(_) => {
                        self.audio_tx.send(action).unwrap();
                    }
                    _ => {
                        self.update_tx.send(action).unwrap();
                    }
                }
            }
        });
    }
}
