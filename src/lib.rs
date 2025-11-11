pub mod messaging;
pub mod model;
pub mod types;
pub mod view;

use crate::messaging::EditAction;
use crate::view::UiApp;
use crossbeam::channel::{Receiver, Sender, unbounded};
use eframe::{NativeOptions, egui};
use egui::ViewportBuilder;

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {
        let (tx, _rx): (Sender<EditAction>, Receiver<EditAction>) = unbounded();

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
