pub trait Controller: Send + Sync + 'static {
    fn next(&mut self);
}
