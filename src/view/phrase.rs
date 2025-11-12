use eframe::egui::{Color32, InputState, Key, RichText, Ui};

use super::view::View;
use crate::messaging::EditAction;
use crate::types::{NUM_STEPS_PER_PHRASE, ChainId, Note, PhraseId, Step};
use crossbeam::channel::Sender;
use crossbeam::channel::bounded;
use std::borrow::Cow;

pub struct Phrase {
    tx: Sender<EditAction>,

    phrase_id: PhraseId,

    // Selection state
    selected_row: usize,

    // Data
    values: Vec<Option<Step>>,
}

impl View for Phrase {
    fn handle_event(&mut self, input: &InputState) {
        let shift_down = input.modifiers.shift;

        if shift_down {
            self.change_selection(input);
        } else {
            self.move_selection(input);
        }
    }

    fn get_selection(&self) -> Option<u8> {
        Some(self.selected_row as u8)
    }

    fn draw(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("PHRASE").heading().color(Color32::LIGHT_BLUE));
        });
        ui.add_space(20.0);

        // Grid for table layout
        egui::Grid::new("chain_grid")
            .num_columns(3)
            .min_col_width(40.0)
            .max_col_width(f32::INFINITY)
            .show(ui, |ui| {
                // Header row
                ui.label(""); // Empty ">" column
                ui.label(""); // Empty row label column
                ui.label(format!("{:0x}", self.phrase_id));
                ui.end_row();

                // Body rows
                for i in 0..NUM_STEPS_PER_PHRASE {
                    ui.label(RichText::new(" ").color(Color32::YELLOW));
                    ui.label(RichText::new(format!("{:02X}", i)).color(Color32::GREEN));

                    let cell = self.render_cell(self.values[i]);
                    let is_selected = i == self.selected_row;
                    let text = RichText::new(cell).size(12.0);
                    if is_selected {
                        ui.label(text.color(Color32::BLACK).background_color(Color32::YELLOW));
                    } else {
                        ui.label(text.color(Color32::WHITE));
                    }

                    ui.end_row();
                }
            });
    }
}

impl Phrase {
    const MAX_CELL_VALUE: usize = 0xFF;
    const BIG_CELL_INCREMENT: isize = 0x10;
    const EMPTY_CELL_DISPLAY: &str = "--";

    pub fn new(tx: Sender<EditAction>, phrase_id: ChainId) -> Self {
        let (reply_tx, reply_rx) = bounded(1); // one-shot channel
        tx.send(EditAction::GetPhraseData {
            phrase_id,
            reply_to: reply_tx,
        })
        .unwrap();

        let phrase_data = reply_rx.recv().unwrap();

        Self {
            tx,
            phrase_id,
            values: phrase_data,
            selected_row: 0,
        }
    }

    fn move_selection(&mut self, input: &InputState) {
        if input.key_pressed(Key::ArrowDown) {
            self.selected_row = (self.selected_row + 1).min(NUM_STEPS_PER_PHRASE - 1);
        }
        if input.key_pressed(Key::ArrowUp) {
            self.selected_row = self.selected_row.saturating_sub(1);
        }
    }

    fn render_cell(&self, value: Option<Step>) -> Cow<'_, str> {
        match value {
            None => Cow::Borrowed(Self::EMPTY_CELL_DISPLAY),
            Some(v) => Cow::Owned(format!("{:02X}", v.note)),
        }
    }

    fn apply_delta(value: Option<Step>, delta: isize) -> Option<Step> {
        match value {
            Some(v) => {
                let new = v.note as isize + delta;
                if new < 0 {
                    None
                } else {
                    Some(Step {note: new.min(Self::MAX_CELL_VALUE as isize) as Note, len: 0 })
                }
            }
            None => {
                if delta == 1 {
                    return Some(Step{note: 0, len: 0});
                } else if delta > 0 {
                    Some(Step{ note: delta.min(Self::MAX_CELL_VALUE as isize) as Note, len: 0})
                } else {
                    None
                }
            }
        }
    }

    fn change_selection(&mut self, input: &egui::InputState) {

        // Figure out how much to increase or decrease
        // the note by.
        let delta = if input.key_pressed(Key::ArrowUp) {
            Some(1)
        } else if input.key_pressed(Key::ArrowDown) {
            Some(-1)
        } else if input.key_pressed(Key::ArrowRight) {
            Some(Self::BIG_CELL_INCREMENT)
        } else if input.key_pressed(Key::ArrowLeft) {
            Some(-Self::BIG_CELL_INCREMENT)
        } else {
            None
        };

        let Some(d) = delta else { return };

        let row = self.selected_row;
        let cell = &mut self.values[row];

        // cell is an Option<Step>

        let new_value = Self::apply_delta(*cell, d);
        *cell = new_value;

        self.tx
            .send(EditAction::SetPhraseStep {
                phrase_id: self.phrase_id,
                index: row as usize,
                step: new_value,
            })
            .unwrap();
    }
}
