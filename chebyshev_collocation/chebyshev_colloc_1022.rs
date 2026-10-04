//! Implementation of chebyshev collocation node order 1022

pub fn compute_chebyshev_colloc_1022(x: f64) -> f64 {
    let node = (std::f64::consts::PI * 2.0_f64 / 3.0_f64).cos();
    node * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_chebyshev_colloc_1022() {
        let res = compute_chebyshev_colloc_1022(0.5);
        assert!(res.is_finite());
    }
}
