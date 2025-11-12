pub type PatternId = u8;
pub type TrackId = u8;
pub type ChainId = u8;
pub type PhraseId = u8;
pub type NoteId = u8; // MIDI note number
pub type Note = u8;

pub const NUM_TRACKS: usize = 8;
pub const NUM_PHRASES_PER_CHAIN: usize = 4;
pub const NUM_STEPS_PER_PHRASE: usize = 16;

/*
 * Each step represents a note
 * or command.
 */

#[derive(Clone, Copy, PartialEq)]
pub struct Step {
    note: u8,
    len: u8,
}

impl Step {
    pub fn new(note: Note, len: u8) -> Self {
        Self { note, len }
    }
}
