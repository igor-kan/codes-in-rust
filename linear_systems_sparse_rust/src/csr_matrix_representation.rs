//! Csr Matrix Representation

pub fn csr_nnz(row_ptr: &[usize]) -> usize { *row_ptr.last().unwrap_or(&0) }
