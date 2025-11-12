use crate::types::{
    ChainId, NUM_PHRASES_PER_CHAIN, NUM_STEPS_PER_PHRASE, NUM_TRACKS, PatternId, PhraseId, Step,
    TrackId,
};
use std::collections::HashMap;

/*
 * A pattern is a single row in the song view.
 * It has a slot for each track, and each slot
 * can either be empty, or can have a chain ID.
 */

pub struct Pattern {
    tracks: [Option<ChainId>; NUM_TRACKS],
}

/*
 * A chain refers to a flexibly-sized list
 * of phrases, up to perhaps 16.
 */

pub struct Chain {
    phrases: [Option<PhraseId>; NUM_PHRASES_PER_CHAIN],
}

impl Chain {
    pub fn new() -> Self {
        Self {
            phrases: [None; NUM_PHRASES_PER_CHAIN],
        }
    }
}

/*
 * A phrase contains 16 steps, which might
 * represent a bar, 4 bars, a quarter bar, etc.
 */

pub struct Phrase {
    steps: [Option<Step>; NUM_STEPS_PER_PHRASE],
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

impl Phrase {
    fn new() -> Self {
        Self {
            steps: [None; NUM_STEPS_PER_PHRASE],
        }
    }
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
    pub fn get_chain_data(&self, chain_id: ChainId) -> Vec<Option<PhraseId>> {
        self.chains
            .get(&chain_id)
            .map(|chain| chain.phrases.iter().cloned().collect())
            .unwrap_or_else(|| Chain::new().phrases.to_vec())
    }

    // Set a phrase in a chain
    pub fn set_chain_phrase(
        &mut self,
        chain_id: ChainId,
        index: usize,
        phrase_id: Option<PhraseId>,
    ) {
        let chain = self.chains.entry(chain_id).or_insert_with(|| Chain::new());

        if index >= NUM_PHRASES_PER_CHAIN {
            panic!(
                "Phrase phrase index out of bounds: index = {index}, chain has {NUM_PHRASES_PER_CHAIN} steps"
            );
        }

        chain.phrases[index] = phrase_id;
    }

    // Get data for a particular phrase
    pub fn get_phrase_data(&self, phrase_id: PhraseId) -> Vec<Option<Step>> {
        self.phrases
            .get(&phrase_id)
            .map(|phrase| phrase.steps.iter().cloned().collect())
            .unwrap_or_else(|| Phrase::new().steps.to_vec())
    }

    // Update a phrase or add a new phrase
    pub fn set_phrase_step(&mut self, phrase_id: PhraseId, index: usize, step: Option<Step>) {
        let phrase = self
            .phrases
            .entry(phrase_id)
            .or_insert_with(|| Phrase::new());

        if index >= NUM_STEPS_PER_PHRASE {
            panic!(
                "Phrase step index out of bounds: index = {index}, phrase has {NUM_STEPS_PER_PHRASE} steps"
            );
        }

        phrase.steps[index] = step;
    }
}
