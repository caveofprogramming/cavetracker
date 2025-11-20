use crate::engine::audio::{Modulator, NodeId, ParamId, Source};

#[derive(Debug, Clone, PartialEq)]
enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Adsr {
    sample_rate: f32,
    target_id: NodeId,
    param_id: ParamId,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    release_start_value: f32,
    stage: EnvelopeStage,
    time: f32,
    param_value: f32,
    base_value: Option<f32>,
}

impl Adsr {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            target_id: 0,
            param_id: 0,
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,
            release_start_value: 0.0,
            stage: EnvelopeStage::Idle,
            time: 0.0,
            param_value: 1.0,
            base_value: None,
        }
    }

    pub fn configure(
        &mut self,
        target_id: NodeId,
        param_id: ParamId,
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
    ) {
        self.target_id = target_id;
        self.param_id = param_id;
        self.attack = attack;
        self.decay = decay;
        self.sustain = sustain;
        self.release = release;
    }

    pub fn get_level(&self) -> f32 {
        self.param_value
    }
}

impl Modulator for Adsr {
    fn tick(&mut self, sources: &mut Vec<Box<dyn Source>>) {
        let dt = 1.0 / self.sample_rate;
        self.time += dt;

        let source = &mut sources[self.target_id];
        let param_value = source.get_param(self.param_id);

        let base_value = *self.base_value.get_or_insert(param_value);

        match self.stage {
            EnvelopeStage::Idle => self.param_value = 0.0,
            EnvelopeStage::Attack => {
                self.param_value = (self.time / self.attack).min(1.0);
                if self.param_value >= 1.0 {
                    self.stage = EnvelopeStage::Decay;
                    self.time = 0.0;
                }
            }
            EnvelopeStage::Decay => {
                let decay_progress = self.time / self.decay;
                self.param_value = 1.0 - decay_progress * (1.0 - self.sustain);
                if decay_progress >= 1.0 {
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => self.param_value = self.sustain,
            EnvelopeStage::Release => {
                let release_progress = self.time / self.release;
                self.param_value = self.release_start_value * (1.0 - release_progress);
                if release_progress >= 1.0 {
                    self.param_value = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }

        source.set_param(self.param_id, base_value * self.param_value);
    }

    fn note_on(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.time = 0.0;
    }

    fn note_off(&mut self) {
        self.stage = EnvelopeStage::Release;
        self.time = 0.0;
        self.release_start_value = self.param_value;
    }

    fn is_active(&self) -> bool {
        self.stage != EnvelopeStage::Idle
    }
}

impl Adsr {
    pub fn is_released(&self) -> bool {
        matches!(self.stage, EnvelopeStage::Idle)
    }

    pub fn is_silent(&self) -> bool {
        self.get_level() < 0.0001 && self.is_released()
    }
}
