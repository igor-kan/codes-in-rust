//! Rejection Sampler

pub fn rejection_bound(target: f64, proposal: f64, m: f64) -> bool { target <= m * proposal }
