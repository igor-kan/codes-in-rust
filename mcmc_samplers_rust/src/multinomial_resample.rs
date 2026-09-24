//! Multinomial Resample

pub fn cumulative_sum(weights: &[f64]) -> Vec<f64> { weights.iter().scan(0.0, |acc, &x| { *acc += x; Some(*acc) }).collect() }
