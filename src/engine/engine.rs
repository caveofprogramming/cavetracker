use crate::messaging::Action;
use crate::UpdateEngine;
use crossbeam::channel::{Sender, Receiver};
use crate::model::Song;
use std::sync::{Mutex, Arc};

pub struct Engine {
    tx: Sender<Action>,
    rx: Receiver<Action>,
}

impl Engine {
    pub fn new(tx: Sender<Action>, rx: Receiver<Action>) -> Self {
        Self {
            tx,
            rx,
        }
    }

    pub fn run(&self) {
        let update_engine = UpdateEngine::new(self.rx.clone(), Arc::new(Mutex::new(Song::new())));
        update_engine.run();

    }
}