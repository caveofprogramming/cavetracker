pub mod audio;
pub mod instrument;
pub mod instrument_manager;
pub mod modulators;
pub mod node;
pub mod sources;
pub mod synth;

pub use audio::Audio;
pub use instrument::Instrument;
pub use instrument_manager::InstrumentManager;
pub use modulators::{Adsr, Lfo, Modulator};
pub use node::*;
pub use sources::{NodeId, ParamId, Sine, Source, param};
pub use synth::Synth;
