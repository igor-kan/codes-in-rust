#[derive(Clone, Debug)]
pub struct AssignRes {
    pub cost: i64,
    pub assign: Vec<usize>,
}

pub const MODULE_TAG: &str = "hungarian_bipartite_rs";
