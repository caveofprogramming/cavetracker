use crate::audio::*;

pub struct NoteFrequency {
    freq: f32,                
    target_node: NodeId,
    target_param: ParamId,      
    triggered: bool,
}

impl NoteFrequency {
    pub fn configure(
        &mut self,
        target_id: NodeId,
        param_id: ParamId,
        freq: f32) {
            self.target_id = target_id,
            self.param_id = param_id,
            self.freq = freq,
        }
}

impl Modulator for NoteFrequency {
    fn new() -> Self {
        Self {
            freq: 0.0,         
            target_node: 0,
            target_param: 0,      
            triggered: false,
        }
    }

    fn fn tick(&mut self, sources: &mut Vec<Box<dyn Source>>) {
        if(!self.triggered) {
            instrument.set_param(self.target_node, self.target_param, self.freq);
            self.triggered = true;
        }
    }

    fn note_on(&mut self) {}

    fn note_off(&mut self) {}

    fn is_active(&self) -> bool {
        true
    }
}
