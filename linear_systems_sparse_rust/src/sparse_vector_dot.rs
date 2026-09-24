//! Sparse Vector Dot

pub fn sparse_dot(idx: &[usize], val: &[f64], dense: &[f64]) -> f64 { idx.iter().zip(val).map(|(&i, &v)| v * dense[i]).sum() }
