use crate::engine::audio::Source;
use downcast_rs::{Downcast, impl_downcast};

pub trait Modulator: Send + Sync + Downcast {
    fn tick(&mut self, sources: &mut Vec<Box<dyn Source>>);
    fn note_on(&mut self) {}
    fn note_off(&mut self) {}
    fn is_active(&self) -> bool {
        true
    }
}

impl_downcast!(Modulator);
