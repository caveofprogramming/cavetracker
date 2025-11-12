pub mod messaging;
pub mod model;
pub mod types;
pub mod view;
pub mod engine;

use crate::messaging::{Action, UpdateEngine};
use crate::model::Song;
use crate::view::UiApp;
use crossbeam::channel::{Receiver, Sender, unbounded};
use eframe::{NativeOptions, egui};
use egui::ViewportBuilder;
use std::sync::Mutex;
use std::sync::Arc;
use crate::engine::Engine;

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {
        let (tx, rx): (Sender<Action>, Receiver<Action>) = unbounded();

        let engine = Engine::new(tx.clone(), rx.clone());
        engine.run();

        let options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_title("CaveTracker Synth Editor")
                .with_inner_size([560.0, 460.0]),
            ..Default::default()
        };

        let _ = eframe::run_native(
            "Blank Black Window",
            options,
            Box::new(|_cc| Ok(Box::new(UiApp::new(tx.clone())))),
        );
    }
}
