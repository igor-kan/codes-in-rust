#[derive(Clone,
    Debug)]
pub struct SNode {
    pub key: i64,
    pub val: i64,
    pub ch: [Option<usize>;
    2],
    pub parent: Option<usize>,
}

pub const MODULE_TAG: &str = "splay_tree_map_rs";
