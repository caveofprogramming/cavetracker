pub trait Source: Send + Sync + 'static {
    fn next_sample(&mut self, input: f32) -> f32;
    fn param_names(&self) -> Vec<String>;
    fn set_param(&mut self, name: &str, value: f32);
}
