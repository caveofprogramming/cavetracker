use crate::model::Song;
use crossbeam::channel::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::types::{ChainId, PatternId, PhraseId, Step, TrackId};

#[cfg(test)]
mod tests {

    use crossbeam::channel::{bounded, unbounded};

    use super::*;
    use serial_test::serial;

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
    fn chain_data() {
        let env = TestEnv::new();

        let tx = env.tx.clone();

        let chain_ids = vec![0, 15, 1];
        let phrase_ids = vec![Some(0), Some(15), None];
        let indices = [0, 1, 3];

        for i in 0..indices.len() {
            let chain_id = chain_ids[i];
            let phrase_id = phrase_ids[i];
            let index = indices[i];

            let _ = tx.send(EditAction::SetChainPhrase {
                chain_id,
                index: index,
                phrase_id: phrase_id,
            });

            let (reply_tx, reply_rx) = bounded(1);

            tx.send(EditAction::GetChainData {
                chain_id,
                reply_to: reply_tx,
            })
            .unwrap();

            let chain = reply_rx.recv().unwrap();

            assert!(chain[index] == phrase_id);
        }
    }

    /*
     * Test setting steps in a phrase, then retrieving all
     * steps and check the steps have been set.
     */

    #[test]
    #[serial]
    fn phrase_data() {
        let env = TestEnv::new();

        let tx = env.tx.clone();

        let phrase_ids = vec![0, 0, 5, 10];
        let indices = vec![0, 3, 15, 2];
        let steps = vec![
            Some(Step::new(4, 8)),
            Some(Step::new(2, 1)),
            Some(Step::new(0, 4)),
            None,
        ];

        for i in 0..steps.len() {
            let phrase_id = phrase_ids[i];
            let index = indices[i];
            let step = steps[i];

            let _ = tx.send(EditAction::SetPhraseStep {
                phrase_id,
                index: index,
                step,
            });
        }

        for i in 0..steps.len() {
            let phrase_id = phrase_ids[i];
            let index = indices[i];
            let step = steps[i];

            let (reply_tx, reply_rx) = bounded(1);

            let _ = tx.send(EditAction::GetPhraseData {
                phrase_id,
                reply_to: reply_tx,
            });

            let phrase = reply_rx.recv().unwrap();

            assert!(phrase[index] == step);
        }
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
        reply_to: Sender<Vec<Option<PhraseId>>>,
    },

    SetChainPhrase {
        chain_id: ChainId,
        index: usize,
        phrase_id: Option<PhraseId>,
    },

    GetPhraseData {
        phrase_id: PhraseId,
        reply_to: Sender<Vec<Option<Step>>>,
    },

    SetPhraseStep {
        phrase_id: PhraseId,
        index: usize,
        step: Option<Step>,
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

                        if let Ok(song_guard) = song.lock() {
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
                        if let Ok(song_guard) = song.lock() {
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
                    EditAction::GetPhraseData {
                        phrase_id,
                        reply_to,
                    } => {
                        if let Ok(song_guard) = song.lock() {
                            let phrase_data = song_guard.get_phrase_data(phrase_id);
                            let _ = reply_to.send(phrase_data);
                        }
                    }
                    EditAction::SetPhraseStep {
                        phrase_id,
                        index,
                        step,
                    } => {
                        if let Ok(mut song_guard) = song.lock() {
                            song_guard.set_phrase_step(phrase_id, index, step);
                        }
                    }
                }
            }
        });
    }
}
