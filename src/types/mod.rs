pub type PatternId = u8;
pub type TrackId = u8;
pub type ChainId = u8;
pub type PhraseId = u8;
pub type NoteId = u8; // MIDI note number

pub const NUM_TRACKS: usize = 8;

/*
 * Each step represents a note
 * or command.
 */
pub struct Step {
    note: u8,
    len: u8,
}
