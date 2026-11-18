#[derive(Clone, Copy, Debug)]
pub struct PTNode {
    pub val: i64,
    pub sum: i64,
    pub prio: u32,
    pub sz: usize,
    pub l: Option<usize>,
    pub r: Option<usize>,
}

pub const MODULE_TAG: &str = "persistent_treap_rs";
