pub struct Lfo {
    target_param: Arc<AtomicF32>,
    modulator: Box<dyn Source>,
    depth: f32,
    center: f32,
}

impl Lfo {
    pub fn new(
        target_param: Arc<AtomicF32>,
        modulator: Box<dyn Source>,
        depth: f32,
        center: f32,
    ) -> Self {
        Self {
            target_param,
            modulator,
            depth,
            center,
        }
    }
}

impl Controller for Lfo {
    fn next(&mut self) {
        let current = self.target_param.load(Ordering::Relaxed);
        let lfo_val = self.modulator.next_sample(0.0);
        let modulated = self.center + lfo_val * self.depth;
        self.target_param.store(modulated, Ordering::Relaxed);
    }
}
