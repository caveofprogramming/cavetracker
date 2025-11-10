use eframe::egui::{Color32, InputState, Key, RichText, Ui};

use super::view::View;
use crate::types::ChainId;
use crate::messaging::EditAction;
use crossbeam::channel::Sender;
use std::borrow::Cow;

const ROWS: usize = 256;
const COLS: usize = 8;

pub struct Song {
    tx: Sender<EditAction>,
    visible_rows: usize,

    // Selection state
    selected_row: usize,
    selected_col: usize,
    min_row: usize,

    // Data
    values: Vec<Vec<Option<ChainId>>>,
}

impl View for Song {
    fn handle_event(&mut self, input: &InputState) {
        let shift_down = input.modifiers.shift;

        if shift_down {
            self.change_selection(input);
        } else {
            self.move_selection(input);
        }
    }

    fn get_selection(&self) -> Option<u8> {
        self.values[self.selected_row][self.selected_col]
    }

    fn draw(&mut self, ui: &mut Ui) {
        let max_row = (self.min_row + self.visible_rows).min(ROWS);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("SONG").heading().color(Color32::LIGHT_BLUE));
        });
        ui.add_space(20.0);

        // Grid for table layout
        egui::Grid::new("song_grid")
            .num_columns(2 + COLS) // ">" column + row label + COLS data columns
            .min_col_width(40.0) // Allow exact column widths
            .max_col_width(f32::INFINITY)
            .show(ui, |ui| {
                // Header row
                ui.label(""); // Empty ">" column
                ui.label(""); // Empty row label column
                for track in 0..COLS {
                    ui.label(
                        RichText::new(format!("{}", track))
                            .strong()
                            .size(12.0) // Fit within row height
                            .color(Color32::LIGHT_BLUE),
                    );
                }
                ui.end_row();

                // Body rows
                for i in self.min_row..max_row {
                    ui.label(RichText::new(" ").color(Color32::YELLOW));
                    ui.label(RichText::new(format!("{:02X}", i)).color(Color32::GREEN));
                    for j in 0..COLS {
                        let cell = self.render_cell(self.values[i][j]);
                        let is_selected = i == self.selected_row && j == self.selected_col;
                        let text = RichText::new(cell).size(12.0); // Consistent font size
                        if is_selected {
                            ui.label(text.color(Color32::BLACK).background_color(Color32::YELLOW));
                        } else {
                            ui.label(text.color(Color32::WHITE));
                        }
                    }
                    ui.end_row();
                }
            });
    }
}

impl Song {
    const MAX_CELL_VALUE: usize = 0xFF;
    const BIG_CELL_INCREMENT: isize = 0x10;
    const EMPTY_CELL_DISPLAY: &str = "--";

    pub fn new(tx: Sender<EditAction>) -> Self {
        Self {
            tx,
            values: vec![vec![None; COLS]; ROWS],
            selected_row: 0,
            selected_col: 0,
            visible_rows: 16,
            min_row: 0,
        }
    }

    fn move_selection(&mut self, input: &InputState) {
        if input.key_pressed(Key::ArrowDown) {
            self.selected_row = (self.selected_row + 1).min(ROWS - 1);

            if self.selected_row >= self.min_row + self.visible_rows {
                self.min_row = (self.min_row + 1).min(ROWS.saturating_sub(self.visible_rows));
            }
        }
        if input.key_pressed(Key::ArrowUp) {
            self.selected_row = self.selected_row.saturating_sub(1);

            if self.selected_row < self.min_row {
                self.min_row = self.min_row.saturating_sub(1);
            }
        }
        if input.key_pressed(Key::ArrowRight) {
            self.selected_col = (self.selected_col + 1).min(COLS - 1);
        }
        if input.key_pressed(Key::ArrowLeft) {
            self.selected_col = self.selected_col.saturating_sub(1);
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
        let col = self.selected_col;
        let cell = &mut self.values[row][col];

        let new_value = Self::apply_delta(*cell, d);
        *cell = new_value;

        self.tx
            .send(EditAction::SetPatternValue {
                pattern_id: row as u8,
                track_id: col as u8,
                chain_id: new_value,
            })
            .unwrap();
    }
}
