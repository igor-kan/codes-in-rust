#[derive(Clone, Debug)]
pub struct AliasTable {
    pub prob: Vec<f64>,
    pub alias: Vec<usize>,
}

pub const MODULE_TAG: &str = "vose_alias_table_rs";
