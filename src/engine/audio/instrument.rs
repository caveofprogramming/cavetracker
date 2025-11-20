use crate::engine::audio::Sine;
use crate::engine::audio::*;

pub struct Instrument {
    sample_rate: f32,
    sources: Vec<Box<dyn Source>>,
    modulators: Vec<Box<dyn Modulator>>,
}

impl Instrument {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            sources: vec![],
            modulators: vec![],
        }
    }

    pub fn from_patch(patch: &Patch) -> Self {
        let mut instrument = Instrument::new(patch.sample_rate);

        let mut node_ids = Vec::with_capacity(patch.nodes.len());

        for node_def in &patch.nodes {
            let id = match node_def {
                NodeDef::Sine(def) => instrument.add_sine(),
                NodeDef::Lfo(def) => instrument.add_lfo(),
                NodeDef::Adsr(def) => instrument.add_adsr(),
            };
            node_ids.push(id);
        }

        for (i, node_def) in patch.nodes.iter().enumerate() {
            let id = node_ids[i];

            match node_def {
                NodeDef::Lfo(def) => {
                    let target = node_ids[def.target_node];
                    instrument.modulators[id]
                        .downcast_mut::<Lfo>()
                        .unwrap()
                        .configure(target, def.target_param, def.freq, def.depth, def.offset);
                }
                NodeDef::Adsr(def) => {
                    let target = node_ids[def.target_node];
                    instrument.modulators[id]
                        .downcast_mut::<Adsr>()
                        .unwrap()
                        .configure(
                            target,
                            def.target_param,
                            def.attack,
                            def.decay,
                            def.sustain,
                            def.release,
                        );
                }
                NodeDef::Sine(_) => {}
            }
        }

        for conn in &patch.connections {
            // e.g. route output of node A into input of node B
            // instrument.connect(conn.from_node, conn.to_node);
        }

        instrument
    }

    fn add_sine(&mut self) -> NodeId {
        let id = self.sources.len();
        self.sources.push(Box::new(Sine::new(self.sample_rate)));
        id
    }

    // Placeholder versions — return the NodeId but don't configure yet
    fn add_lfo(&mut self) -> NodeId {
        let id = self.modulators.len();
        self.modulators.push(Box::new(Lfo::new(self.sample_rate)));
        id
    }

    fn add_adsr(&mut self) -> NodeId {
        let id = self.modulators.len();
        self.modulators.push(Box::new(Adsr::new(self.sample_rate)));
        id
    }

    pub fn configure_lfo(
        &mut self,
        lfo_id: NodeId,
        target_id: NodeId,
        target_param: ParamId,
        freq: f32,
        depth: f32,
        offset: f32,
    ) {
        if let Some(lfo) = self.modulators[lfo_id].downcast_mut::<Lfo>() {
            lfo.configure(target_id, target_param, freq, depth, offset);
        }
    }

    pub fn configure_adsr(
        &mut self,
        adsr_id: NodeId,
        target_id: NodeId,
        target_param: ParamId,
        a: f32,
        d: f32,
        s: f32,
        r: f32,
    ) {
        if let Some(adsr) = self.modulators[adsr_id].downcast_mut::<Adsr>() {
            adsr.configure(target_id, target_param, a, d, s, r);
        }
    }

    pub fn is_released(&self) -> bool {
        self.modulators.iter().all(|m| {
            m.downcast_ref::<Adsr>()
                .map(|adsr| adsr.is_released())
                .unwrap_or(true)
        })
    }

    pub fn is_silent(&self) -> bool {
        self.modulators.iter().all(|m| {
            m.downcast_ref::<Adsr>()
                .map(|adsr| adsr.get_level() < 0.0001 && adsr.is_released())
                .unwrap_or(true)
        })
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        for source in &mut self.sources {
            source.set(note, velocity);
        }

        for modulator in &mut self.modulators {
            modulator.note_on();
        }
    }

    pub fn note_off(&mut self) {
        for modulator in &mut self.modulators {
            modulator.note_off();
        }
    }

    pub fn next(&mut self) -> f32 {
        // 1. Update modulators
        for modulator in &mut self.modulators {
            modulator.tick(&mut self.sources);
        }

        // 2. Sum source outputs
        let mut sum = 0.0;
        for source in &mut self.sources {
            sum += source.next();
        }

        // 3. Normalize (optional)
        if !self.sources.is_empty() {
            sum /= self.sources.len() as f32;
        }

        sum
    }
}
