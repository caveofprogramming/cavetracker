use eframe::egui::{Color32, InputState, Key, RichText, Ui};

use super::view::View;
use crate::messaging::Action;
use crate::types::NUM_PHRASES_PER_CHAIN;
use crate::types::{ChainId, PhraseId};
use crossbeam::channel::Sender;
use crossbeam::channel::bounded;
use std::borrow::Cow;

pub struct Chain {
    tx: Sender<Action>,

    chain_id: ChainId,

    // Selection state
    selected_row: usize,

    // Data
    values: Vec<Option<PhraseId>>,
}

impl View for Chain {
    fn handle_event(&mut self, input: &InputState) {
        let shift_down = input.modifiers.shift;

        if shift_down {
            self.change_selection(input);
        } else {
            self.move_selection(input);
        }
    }

    fn get_selection(&self) -> Option<u8> {
        self.values[self.selected_row]
    }

    fn draw(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("CHAIN").heading().color(Color32::LIGHT_BLUE));
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
                ui.label(format!("{:0x}", self.chain_id));
                ui.end_row();

                // Body rows
                for i in 0..NUM_PHRASES_PER_CHAIN {
                    ui.label(RichText::new(" ").color(Color32::YELLOW));
                    ui.label(RichText::new(format!("{:02X}", i)).color(Color32::GREEN));

                    let cell = if self.values.len() < i {
                        self.render_cell(None)
                    } else {
                        self.render_cell(self.values[i])
                    };

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

impl Chain {
    const MAX_CELL_VALUE: usize = 0xFF;
    const BIG_CELL_INCREMENT: isize = 0x10;
    const EMPTY_CELL_DISPLAY: &str = "--";

    pub fn new(tx: Sender<Action>, chain_id: ChainId) -> Self {
        let (reply_tx, reply_rx) = bounded(1);

        tx.send(Action::GetChainData {
            chain_id,
            reply_to: reply_tx,
        })
        .unwrap();

        let chain_data = reply_rx.recv().unwrap();

        Self {
            tx,
            chain_id,
            values: chain_data,
            selected_row: 0,
        }
    }

    fn move_selection(&mut self, input: &InputState) {
        if input.key_pressed(Key::ArrowDown) {
            self.selected_row = (self.selected_row + 1).min(NUM_PHRASES_PER_CHAIN - 1);
        }
        if input.key_pressed(Key::ArrowUp) {
            self.selected_row = self.selected_row.saturating_sub(1);
        }
    }

    fn render_cell(&self, value: Option<u8>) -> Cow<'_, str> {
        match value {
            None => Cow::Borrowed(Self::EMPTY_CELL_DISPLAY),
            Some(v) => Cow::Owned(format!("{:02X}", v)),
        }
    }

    fn apply_delta(value: Option<u8>, delta: isize) -> Option<u8> {
        match value {
            Some(v) => {
                let new = v as isize + delta;
                if new < 0 {
                    None
                } else {
                    Some(new.min(Self::MAX_CELL_VALUE as isize) as u8)
                }
            }
            None => {
                if delta == 1 {
                    return Some(0);
                } else if delta > 0 {
                    Some(delta.min(Self::MAX_CELL_VALUE as isize) as u8)
                } else {
                    None
                }
            }
        }
    }

    fn change_selection(&mut self, input: &egui::InputState) {
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

        let new_value = Self::apply_delta(*cell, d);
        *cell = new_value;

        self.tx
            .send(Action::SetChainPhrase {
                chain_id: self.chain_id,
                index: row as usize,
                phrase_id: new_value,
            })
            .unwrap();
    }
}
