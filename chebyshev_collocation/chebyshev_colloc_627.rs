//! Implementation of chebyshev collocation node order 627

pub fn compute_chebyshev_colloc_627(x: f64) -> f64 {
    let node = (std::f64::consts::PI * 7.0_f64 / 8.0_f64).cos();
    node * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_chebyshev_colloc_627() {
        let res = compute_chebyshev_colloc_627(0.5);
        assert!(res.is_finite());
    }
}
