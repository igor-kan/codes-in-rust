//! Spmv Csr Kernel

pub fn spmv_row(values: &[f64], cols: &[usize], x: &[f64]) -> f64 { values.iter().zip(cols).map(|(&v, &c)| v * x[c]).sum() }
