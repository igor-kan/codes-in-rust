#[derive(Clone,
    Debug)]
pub struct GhEdge {
    pub u: usize,
    pub v: usize,
    pub cap: i64,
}

pub const MODULE_TAG: &str = "gomory_hu_tree_rs";
