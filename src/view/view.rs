use egui::{InputState, Ui};

pub trait View {
    fn handle_event(&mut self, input: &InputState);
    fn draw(&mut self, ui: &mut Ui);
    fn get_selection(&self) -> Option<u8>;
}
