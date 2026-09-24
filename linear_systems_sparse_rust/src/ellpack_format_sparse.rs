//! Ellpack Format Sparse

pub fn ellpack_index(row: usize, k: usize, max_nnz: usize) -> usize { row * max_nnz + k }
