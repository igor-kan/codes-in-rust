#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub to: usize,
    pub rev: usize,
    pub cap: i64,
}

pub const MODULE_TAG: &str = "dinic_maxflow_rs";
