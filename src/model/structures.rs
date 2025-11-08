use crate::types::{ChainId, NUM_TRACKS, PatternId, PhraseId, Step, TrackId};
use std::collections::HashMap;

/*
 * A pattern is a single row in the song view.
 * It has a slot for each track, and each slot
 * can either be empty, or can have a chain ID.
 */

pub struct Pattern {
    tracks: [Option<ChainId>; 8],
}

/*
 * A chain refers to a flexibly-sized list
 * of phrases, up to perhaps 16.
 */

pub struct Chain {
    phrases: Vec<PhraseId>,
}

/*
 * A phrase contains 16 steps, which might
 * represent a bar, 4 bars, a quarter bar, etc.
 */

pub struct Phrase {
    steps: [Option<Step>; 16],
}

/*
 * Song stores all necessary
 * patterns, chains and phrases.
 * The number of patterns is flexible.
 */

pub struct Song {
    patterns: Vec<Pattern>,
    pub chains: HashMap<ChainId, Chain>,
    pub phrases: HashMap<PhraseId, Phrase>,
}

impl Song {
    pub fn new() -> Self {
        Self {
            patterns: vec![],
            chains: HashMap::new(),
            phrases: HashMap::new(),
        }
    }

    // Get data for all patterns.
    pub fn get_pattern_data(&self) -> Vec<Vec<Option<ChainId>>> {
        self.patterns
            .iter()
            .map(|pattern| pattern.tracks.to_vec())
            .collect()
    }

    // Get data for an individual pattern
    pub fn get_pattern(&self, pattern_id: PatternId) -> Vec<Option<ChainId>> {
        let index = pattern_id as usize;
        self.patterns
            .get(index)
            .map(|pattern| pattern.tracks.to_vec())
            .unwrap_or_else(|| vec![None; NUM_TRACKS])
    }

    pub fn update_pattern(
        &mut self,
        pattern_id: PatternId,
        track_id: TrackId,
        chain_id: Option<ChainId>,
    ) {
        let pattern_index = pattern_id as usize;
        let track_index = track_id as usize;

        // Ensure the pattern exists
        while self.patterns.len() <= pattern_index {
            self.patterns.push(Pattern {
                tracks: [None; NUM_TRACKS],
            });
        }

        // Update the pattern's track with the new chain ID
        self.patterns[pattern_index].tracks[track_index] = chain_id;

        println!(
            "Updated pattern {} track {} with chain {:?}",
            pattern_id, track_id, chain_id
        );
    }

    // Get data for a particular chain
    pub fn get_chain_data(&self, chain_id: ChainId) -> Vec<PhraseId> {
        self.chains
            .get(&chain_id)
            .map(|chain| chain.phrases.iter().cloned().collect())
            .unwrap_or_else(Vec::new)
    }

    // Update a chain or add a new chain
    pub fn set_chain_data(&mut self, chain_id: ChainId, phrases: Vec<PhraseId>) {
        self.chains.insert(chain_id, Chain { phrases });
    }
}
