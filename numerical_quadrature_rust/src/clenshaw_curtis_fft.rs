//! Clenshaw Curtis Fft

pub fn cc_chebyshev_node(k: usize, n: usize) -> f64 { ((k as f64) * std::f64::consts::PI / (n as f64)).cos() }
