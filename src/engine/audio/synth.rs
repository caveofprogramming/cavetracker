use crate::engine::audio::*;

pub struct Synth {
    patch: Patch,
    voices: Vec<Voice>,   // Active + free voices
    active_voices: usize, // Number of currently playing voices
    next_voice: usize,    // Round-robin / steal from here
}

pub struct Voice {
    instrument: Instrument,
    note: u8, // MIDI note number
    velocity: u8,
    is_active: bool,
}

impl Synth {
    pub fn new(patch: Patch, max_polyphony: usize) -> Self {
        let mut voices = Vec::with_capacity(max_polyphony);
        for _ in 0..max_polyphony {
            voices.push(Voice {
                instrument: Instrument::from_patch(&patch),
                note: 0,
                velocity: 0,
                is_active: false,
            });
        }

        Self {
            patch,
            voices,
            active_voices: 0,
            next_voice: 0,
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let len = self.voices.len();
        if len == 0 {
            return;
        }

        let start = self.next_voice;
        let mut chosen = start;

        // Cache the length → no more borrow of self.voices inside the loop
        for i in 0..len {
            let idx = (start + i) % len;
            if !self.voices[idx].is_active {
                chosen = idx;
                break;
            }
        }

        // Now safe to use self.voices again
        let voice = &mut self.voices[chosen];
        voice.instrument = Instrument::from_patch(&self.patch);
        voice.instrument.note_on(note, velocity);
        voice.note = 0;
        voice.velocity = 0;
        voice.is_active = true;

        self.next_voice = (chosen + 1) % len;
        if self.active_voices < len {
            self.active_voices += 1;
        }
    }

    pub fn note_off(&mut self, note: u8) {
        for voice in &mut self.voices {
            if voice.is_active && voice.note == note {
                voice.instrument.note_off();
            }
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut sum = 0.0;
        let mut active = 0;

        for voice in &mut self.voices {
            if voice.is_active {
                let sample = voice.instrument.next();
                if voice.instrument.is_silent() {
                    voice.is_active = false;
                } else {
                    sum += sample;
                    active += 1;
                }
            }
        }

        self.active_voices = self.voices.iter().filter(|v| v.is_active).count();

        if active > 0 { sum / active as f32 } else { 0.0 }
    }
}
