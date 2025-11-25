use crate::UpdateEngine;
use crate::engine::Dispatcher;
use crate::engine::audio::*;
use crate::messaging::Action;
use crate::model::Song;
use crossbeam::channel::{Receiver, Sender, unbounded};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Engine {
    tx: Sender<Action>,
    rx: Receiver<Action>,
}

impl Engine {
    pub fn new(tx: Sender<Action>, rx: Receiver<Action>) -> Self {
        Self { tx, rx }
    }

    pub fn run(&self) {
        let (update_tx, update_rx) = unbounded::<Action>();
        let (audio_tx, audio_rx) = unbounded::<Action>();

        // Dispatcher: main rx → update/audio
        Dispatcher::new(self.rx.clone(), update_tx.clone(), audio_tx.clone()).run();

        let update_engine = UpdateEngine::new(update_rx, Arc::new(Mutex::new(Song::new())));
        update_engine.run();

        let instrument_manager = Arc::new(Mutex::new(InstrumentManager::new()));

        thread::spawn(move || {
            let mut audio_engine = Audio::new(instrument_manager.clone(), audio_tx.clone());
            instrument_manager.lock().set_sample_rate(audio_engine.get_sample_rate() as f32);
            instrument_manager.lock().add_synth();

            let mut running = false;

            while let Ok(action) = audio_rx.recv() {
                match action {
                    Action::TogglePlayPhrase(phrase_id) => {
                        if !running {
                            println!("Play phrase {}", phrase_id);
                            audio_engine.start();
                            running = true;
                        } else {
                            println!("Stop phrase {}", phrase_id);
                            audio_engine.stop();
                            running = false;
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}
