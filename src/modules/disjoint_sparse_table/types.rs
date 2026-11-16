#[derive(Debug, Clone)]
pub struct DisjointSparseTable {
    pub orig_len: usize,
    pub pad_len: usize,
    pub levels: usize,
    pub modulus: u64,
    pub data: Vec<u64>,
    pub table: Vec<Vec<u64>>,
}
