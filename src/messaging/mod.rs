use crate::model::Song;
use crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::types::{ChainId, NoteId, PatternId, PhraseId, TrackId};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pattern_data() {
        let song = Arc::new(Mutex::new(Song::new()));

        let pattern_id = 10;
        let chain_id = 15;
        let track_id = 5;

        let pattern = {
            let mut guard = song.lock().expect("Failed to lock song");
            guard.update_pattern(pattern_id, track_id, Some(chain_id));
        };

        let (tx, rx): (Sender<EditAction>, Receiver<EditAction>) = unbounded();

        let update_engine = UpdateEngine::new(rx.clone(), song.clone());
        update_engine.run();

        let (reply_tx, reply_rx) = bounded(1);

        tx.send(EditAction::GetPatternData { reply_to: reply_tx })
            .unwrap();

        let pattern_data = reply_rx.recv().unwrap();

        let pattern = {
            let guard = song.lock().expect("Failed to lock song");
            guard.get_pattern(pattern_id);
        };

        assert!(pattern_data.len() > pattern_id as usize); 
        assert_eq!(pattern_data[pattern_id as usize][track_id as usize], Some(chain_id));

        drop(tx); // close channel
    }
}

pub enum EditAction {
    /*
     * Get all pattern data in convenient form.
     */
    GetPatternData {
        reply_to: Sender<Vec<Vec<Option<ChainId>>>>,
    },

    GetChainData {
        chain_id: ChainId,
        reply_to: Sender<Vec<Option<PhraseId>>>,
    },

    GetPhraseData {
        phrase_id: PhraseId,
        reply_to: Sender<Vec<Option<NoteId>>>,
    },

    SetPatternValue {
        pattern: PatternId,
        track: TrackId,
        chain: Option<ChainId>,
    },

    SetChainValue {
        chain: ChainId,
        index: u8,
        phrase: Option<PhraseId>,
    },

    SetPhraseValue {
        phrase: PhraseId,
        index: u8,
        note: Option<NoteId>,
    },
}

pub struct UpdateEngine {
    rx: Receiver<EditAction>,
    song: Arc<Mutex<Song>>,
}

impl UpdateEngine {
    pub fn new(rx: Receiver<EditAction>, song: Arc<Mutex<Song>>) -> Self {
        Self { rx, song }
    }

    pub fn run(&self) {
        let rx = self.rx.clone();
        let song = self.song.clone();

        thread::spawn(move || {
            while let Ok(action) = rx.recv() {
                match action {
                    EditAction::GetPatternData { reply_to } => {
                        println!("Get pattern data");

                        if let Ok(mut song_guard) = song.lock() {
                            let pattern_data = song_guard.get_pattern_data();
                            let _ = reply_to.send(pattern_data);
                        }
                    }
                    EditAction::SetPatternValue {
                        pattern,
                        track,
                        chain,
                    } => {
                        if let Ok(mut song_guard) = song.lock() {
                            song_guard.update_pattern(pattern, track, chain);
                        }
                    }
                    EditAction::SetChainValue {
                        chain,
                        index,
                        phrase,
                    } => {
                        if let Ok(mut song_guard) = song.lock() {
                            //song_guard.update_chain(chain, index, phrase);
                        }
                    }
                    EditAction::SetPhraseValue {
                        phrase,
                        index,
                        note,
                    } => {
                        if let Ok(song_guard) = song.lock() {
                            //song_guard.update_phrase(phrase, index, note);
                        }
                    }
                    EditAction::GetChainData { chain_id, reply_to } => {
                        /*
                        if let Ok(mut song_guard) = song.lock() {
                            let chain_data = song_guard.get_chain_data(chain_id);
                            let _ = reply_to.send(chain_data);
                        }
                        */
                    }
                    EditAction::GetPhraseData {
                        phrase_id,
                        reply_to,
                    } => {
                        if let Ok(song_guard) = song.lock() {
                            /*
                            let phrase_data = song_guard.get_phrase_data(phrase_id);
                            let reply: Vec<_> = phrase_data.iter().map(|s| s.note).collect();
                            let _ = reply_to.send(reply);
                            */
                        }
                    }
                }
            }
        });
    }
}
