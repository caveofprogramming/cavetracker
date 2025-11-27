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

    pub fn run(&mut self) {
        /*
        thread::spawn(move || {
            while let Ok(action) = self.rx.recv() {
                match &action {
                    Action::PlayPhrase(_) => {
                        if self.is_playing {
                            self.sequencer_tx.send(Action::StopAudio).unwrap();
                            self.audio_tx.send(Action::StopAudio).unwrap();
                            self.is_playing = false;
                        } else {
                            self.sequencer_tx.send(action).unwrap();
                            self.audio_tx.send(Action::PlayAudio).unwrap();
                            self.is_playing = true;
                        }
                    }
                    _ => {
                        self.update_tx.send(action).unwrap();
                    }
                }
            }
        });
        */
    }
}
