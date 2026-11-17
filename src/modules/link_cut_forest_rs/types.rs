#[derive(Clone,
    Debug)]
pub struct LctNode {
    pub p: Option<usize>,
    pub ch: [Option<usize>;
    2],
    pub rev: bool,
    pub val: u64,
    pub xsum: u64,
}

pub const MODULE_TAG: &str = "link_cut_forest_rs";
