pub mod messaging;
pub mod model;
pub mod types;
pub mod view;

use crate::messaging::EditAction;
use eframe::{NativeOptions, egui};
use egui::ViewportBuilder;
use crossbeam::channel::{Receiver, Sender, unbounded};
use crate::view::UiApp;

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {

        let (tx, rx):(Sender<EditAction>, Receiver<EditAction>) = unbounded();

        
        let options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_title("CaveTracker Synth Editor")
                .with_inner_size([560.0, 460.0]),
            ..Default::default()
        };

        eframe::run_native(
            "Blank Black Window",
            options,
            Box::new(|_cc| Ok(Box::new(UiApp::new(tx.clone())))),
        );
    }
}
