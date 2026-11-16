use crate::types::DisjointSparseTable;

pub fn batch_range_products(t: &DisjointSparseTable, qs: &[(usize, usize)]) -> Vec<u64> {
    qs.iter().map(|&(l, r)| t.query_inclusive(l, r)).collect()
}
