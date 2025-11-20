use crate::engine::audio::*;

#[derive(Clone, Debug)]
pub struct Patch {
    pub sample_rate: f32,
    pub nodes: Vec<NodeDef>,
    pub connections: Vec<Connection>,
}

#[derive(Clone, Debug)]
pub enum NodeDef {
    Sine(SineDef),
    Lfo(LfoDef),
    Adsr(AdsrDef),
}

#[derive(Clone, Debug)]
pub struct SineDef {}

#[derive(Clone, Debug)]
pub struct LfoDef {
    pub freq: f32,
    pub depth: f32,
    pub offset: f32,
    pub target_node: NodeId,
    pub target_param: ParamId,
}

#[derive(Clone, Debug)]
pub struct AdsrDef {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub target_param: ParamId,
    pub target_node: NodeId,
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub from_node: NodeId,
    pub to_node: NodeId,
}
