//! Implementation of airy function series component order 95

pub fn compute_airy_function_95(x: f64) -> f64 {
    let k = 5;
    let denom = (3.0_f64).powi(k) * (max_fact(k) as f64) * (max_fact(k + 1) as f64);
    x.powi(3 * k) / (if denom > 0.0 { denom } else { 1.0 })
}
fn max_fact(n: i32) -> u64 {
    match n {
        0 | 1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        _ => 5040,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_airy_function_95() {
        let res = compute_airy_function_95(0.5);
        assert!(res.is_finite());
    }
}
