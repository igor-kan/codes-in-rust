#[derive(Clone,
    Debug)]
pub struct MatchOut {
    pub pairs: usize,
    pub partner: Vec<Option<usize>>,
}

pub const MODULE_TAG: &str = "blossom_matching_rs";
