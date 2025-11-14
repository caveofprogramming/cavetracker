use super::source::Source;
use std::collections::HashMap;

pub struct Instrument {
    sources: Vec<Box<dyn Source>>,
}

impl Instrument {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    pub fn add_source(&mut self, source: Box<dyn Source>) {
        self.sources.push(source);
    }
}

impl Source for Instrument {
    fn next_sample(&mut self, input: f32) -> f32 {
        let mut sample: f32 = input;

        for source in &mut self.sources {
            sample = source.next_sample(input);
        }

        sample
    }

    fn param_names(&self) -> Vec<String> {
        vec![]
    }

    fn set_param(&mut self, name: &str, value: f32) {
        
    }
}
