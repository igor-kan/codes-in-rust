//! Csc Matrix Representation

pub fn csc_cols(col_ptr: &[usize]) -> usize { col_ptr.len().saturating_sub(1) }
