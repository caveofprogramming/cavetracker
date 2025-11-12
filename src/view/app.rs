use eframe::{
    App,
    egui::{
        CentralPanel, Context, Direction, FontData, FontDefinitions, FontFamily, Frame, InputState,
        Layout, Margin, Ui,
    },
};

use crate::messaging::Action;
use crate::view::Chain;
use crate::view::Phrase;
use crate::view::Song;
use crate::view::View;
use crossbeam::channel::Sender;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

enum ViewMode {
    Song,
    Chain,
    Phrase,
}

pub struct UiApp {
    tx: Sender<Action>,
    font_loaded: bool,
    song_view: Rc<RefCell<dyn View>>,
    view: Rc<RefCell<dyn View>>,
    mode: ViewMode,
}

fn load_custom_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    // Load the font file
    fonts.font_data.insert(
        "my_font".to_owned(),
        Arc::new(FontData::from_owned(
            include_bytes!("../assets/PressStart2P-Regular.ttf").to_vec(),
        )),
    );

    // Use it as the default proportional font
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(fonts);
}

impl UiApp {
    pub fn new(tx: Sender<Action>) -> Self {
        let view = Rc::new(RefCell::new(Song::new(tx.clone())));
        Self {
            tx: tx.clone(),
            font_loaded: false,
            song_view: view.clone(),
            view,
            mode: ViewMode::Song,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        self.view.borrow_mut().draw(ui);
    }

    pub fn handle_event(&mut self, input: &InputState) {
        if input.key_pressed(egui::Key::Enter) {
            let selected_value = self.view.borrow().get_selection();

            match self.mode {
                ViewMode::Song => {
                    if let Some(chain_id) = selected_value {
                        self.mode = ViewMode::Chain;
                        self.view = Rc::new(RefCell::new(Chain::new(self.tx.clone(), chain_id)));
                    }
                }
                ViewMode::Chain => {
                    if let Some(phrase_id) = selected_value {
                        self.mode = ViewMode::Phrase;
                        self.view = Rc::new(RefCell::new(Phrase::new(self.tx.clone(), phrase_id)));
                    }
                }
                ViewMode::Phrase => {
                    self.view = self.song_view.clone();
                    self.mode = ViewMode::Song;
                }
            }
        } else if input.key_pressed(egui::Key::Space) {
        }

        self.view.borrow_mut().handle_event(input);
    }
}

impl App for UiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.font_loaded {
            load_custom_font(ctx);
            self.font_loaded = true;
        }

        ctx.input(|i| self.handle_event(i));

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                Frame::none()
                    .inner_margin(Margin::symmetric(20.0, 20.0))
                    .show(ui, |ui| self.draw(ui));
            });
        });
    }
}
