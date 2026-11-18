#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObstResult {
    pub cost: i64,
    pub root: usize,
}

pub const MODULE_TAG: &str = "knuth_yao_obst_rs";
