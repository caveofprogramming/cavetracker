use crate::UpdateEngine;
use crate::messaging::Action;
use crate::model::Song;
use crossbeam::channel::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use crate::engine::Audio;
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
        let update_engine = UpdateEngine::new(self.rx.clone(), Arc::new(Mutex::new(Song::new())));
        update_engine.run();

        thread::spawn(|| {
            let mut audio_engine = Audio::new();
            
            loop {
                audio_engine.start();
                thread::sleep(Duration::from_millis(1000));
                audio_engine.stop();
                thread::sleep(Duration::from_millis(1000));
            }

        });
    }
}
