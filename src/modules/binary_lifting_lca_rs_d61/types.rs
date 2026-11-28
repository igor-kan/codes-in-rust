#[derive(Clone, Debug)]
pub struct LcaTable {
    pub depth: Vec<usize>,
    pub up: Vec<Vec<usize>>,
}

impl LcaTable {
    pub fn node_count(&self) -> usize {
        self.depth.len()
    }
}

pub const MODULE_TAG: &str = "binary_lifting_lca_rs_d61";
