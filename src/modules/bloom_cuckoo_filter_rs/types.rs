#[derive(Debug, Clone)]
pub struct CuckooFilter {
    pub num_buckets: usize,
    pub buckets: Vec<[u16; 4]>,
    pub count: usize,
    pub seed: u64,
}
