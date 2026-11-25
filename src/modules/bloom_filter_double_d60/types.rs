#[derive(Clone, Debug)]
pub struct BloomConfig {
    pub word_count: usize,
    pub hashes: usize,
}

pub const MODULE_TAG: &str = "bloom_filter_double_d60";
