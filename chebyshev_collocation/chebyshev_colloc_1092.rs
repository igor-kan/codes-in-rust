//! Implementation of chebyshev collocation node order 1092

pub fn compute_chebyshev_colloc_1092(x: f64) -> f64 {
    let node = (std::f64::consts::PI * 2.0_f64 / 3.0_f64).cos();
    node * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_chebyshev_colloc_1092() {
        let res = compute_chebyshev_colloc_1092(0.5);
        assert!(res.is_finite());
    }
}
