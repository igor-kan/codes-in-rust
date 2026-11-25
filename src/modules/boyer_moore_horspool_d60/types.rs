#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchSummary {
    pub hits: usize,
    pub pattern_len: usize,
}

pub const MODULE_TAG: &str = "boyer_moore_horspool_d60";
