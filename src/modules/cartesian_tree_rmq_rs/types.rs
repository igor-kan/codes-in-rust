#[derive(Clone, Debug)]
pub struct CartTree {
    pub root: Option<usize>,
    pub left: Vec<Option<usize>>,
    pub right: Vec<Option<usize>>,
}

pub const MODULE_TAG: &str = "cartesian_tree_rmq_rs";
