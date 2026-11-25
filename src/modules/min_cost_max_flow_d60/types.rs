#[derive(Clone, Debug)]
pub struct McEdge {
    pub to: usize,
    pub rev: usize,
    pub cap: i64,
    pub cost: i64,
}

pub const MODULE_TAG: &str = "min_cost_max_flow_d60";
