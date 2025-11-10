use crate::model::Song;
use crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use serial_test::serial;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::types::{ChainId, NoteId, PatternId, PhraseId, Step, TrackId};

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEnv {
        tx: Sender<EditAction>,
        rx: Receiver<EditAction>,
    }

    impl TestEnv {
        fn new() -> Self {
            let song = Arc::new(Mutex::new(Song::new()));
            let (tx, rx): (Sender<EditAction>, Receiver<EditAction>) = unbounded();
            let update_engine = UpdateEngine::new(rx.clone(), song.clone());
            update_engine.run();

            Self {
                tx: tx.clone(),
                rx: rx.clone(),
            }
        }
    }

    /*
     * Test setting a pattern chain_id, then retrieving all
     * patterns and checking the value has been set.
     */

    #[test]
    #[serial]
    fn pattern_data() {
        let env = TestEnv::new();

        let tx = env.tx.clone();

        let pattern_id = 10;
        let chain_id = 15;
        let track_id = 5;

        let _ = tx.send(EditAction::SetPatternValue {
            pattern_id,
            track_id,
            chain_id: Some(chain_id),
        });

        let (reply_tx, reply_rx) = bounded(1);

        tx.send(EditAction::GetPatternData { reply_to: reply_tx })
            .unwrap();

        let pattern_data = reply_rx.recv().unwrap();

        assert!(pattern_data.len() > pattern_id as usize);
        assert_eq!(
            pattern_data[pattern_id as usize][track_id as usize],
            Some(chain_id)
        );
    }

    /*
     * Test setting phrase IDs in a chain, then retrieving all
     * chains and check the phrase IDs have been set.
     */

    #[test]
    #[serial]
    fn get_chain_data() {
        let env = TestEnv::new();

        let tx = env.tx.clone();

        let chain_id = 15;
        let phrase_id1 = 10;
        let index1 = 0;
        let phrase_id2 = 15;
        let index2 = 1;

        let _ = tx.send(EditAction::SetChainPhrase {
            chain_id,
            index: index1,
            phrase_id: phrase_id1,
        });

        let _ = tx.send(EditAction::SetChainPhrase {
            chain_id,
            index: index2,
            phrase_id: phrase_id2,
        });

        let (reply_tx, reply_rx) = bounded(1);

        tx.send(EditAction::GetChainData {
            chain_id,
            reply_to: reply_tx,
        })
        .unwrap();

        let chain = reply_rx.recv().unwrap();

        assert!(chain[index1] == phrase_id1);
        assert!(chain[index2] == phrase_id2);
    }
}

pub enum EditAction {
    /*
     * Get all pattern data in convenient form.
     */
    GetPatternData {
        reply_to: Sender<Vec<Vec<Option<ChainId>>>>,
    },

    /*
     * Set the chain ID of a particular pattern
     * for a particular track.
     */
    SetPatternValue {
        pattern_id: PatternId,
        track_id: TrackId,
        chain_id: Option<ChainId>,
    },

    /*
     * Get the list of phrases that
     * compose a particular chain.
     */
    GetChainData {
        chain_id: ChainId,
        reply_to: Sender<Vec<PhraseId>>,
    },

    SetChainPhrase {
        chain_id: ChainId,
        index: usize,
        phrase_id: PhraseId,
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
                        pattern_id,
                        track_id,
                        chain_id,
                    } => {
                        if let Ok(mut song_guard) = song.lock() {
                            song_guard.update_pattern(pattern_id, track_id, chain_id);
                        }
                    }
                    EditAction::GetChainData { chain_id, reply_to } => {
                        if let Ok(mut song_guard) = song.lock() {
                            let chain_data = song_guard.get_chain_data(chain_id);
                            let _ = reply_to.send(chain_data);
                        }
                    }
                    EditAction::SetChainPhrase {
                        chain_id,
                        index,
                        phrase_id,
                    } => {
                        if let Ok(mut song_guard) = song.lock() {
                            song_guard.set_chain_phrase(chain_id, index, phrase_id);
                        }
                    }
                }
            }
        });
    }
}
