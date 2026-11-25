#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KvEntry {
    pub key: i64,
    pub val: i64,
}

pub const MODULE_TAG: &str = "bplus_leaf_index_d60";
