use crate::UpdateEngine;
use crate::engine::Dispatcher;
use crate::engine::Sequencer;
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
        let (sequencer_tx, sequencer_rx) = unbounded::<Action>();

        /*
         * InstrumentManager supplies the actual samples
         */

        // Dispatcher: main rx → update/audio/sequencer
        let mut dispatcher = Dispatcher::new(
            self.rx.clone(),
            update_tx.clone(),
            sequencer_tx.clone(),
            audio_tx.clone(),
        );
        dispatcher.run();

        let sequencer = Arc::new(Mutex::new(Sequencer::new(
            self.tx.clone(),
            sequencer_rx,
            44100,
            120.0,
        )));

        let update_engine = UpdateEngine::new(update_rx, Arc::new(Mutex::new(Song::new())));
        update_engine.run();

        let instrument_manager = Arc::new(Mutex::new(InstrumentManager::new()));

        thread::spawn(move || {
            let mut audio_engine = Audio::new(
                audio_tx.clone(),
                instrument_manager.clone(),
                sequencer.clone(),
            );

            instrument_manager
                .lock()
                .set_sample_rate(audio_engine.get_sample_rate() as f32);
            instrument_manager.lock().add_synth();

            while let Ok(action) = audio_rx.recv() {
                match action {
                    Action::PlayPhrase(phrase_id) => {
                        if audio_engine.is_playing() {
                            println!("stop");
                            audio_engine.stop();
                        } else {
                            println!("start");
                            audio_engine.start();
                        }
                    }

                    _ => {}
                }
            }
        });
    }
}
