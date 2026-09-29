//! Implementation of riemann zeta series component order 4

pub fn compute_riemann_zeta_4(x: f64) -> f64 {
    let sign = -1.0_f64;
    sign / ((4 as f64).powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_riemann_zeta_4() {
        let res = compute_riemann_zeta_4(0.5);
        assert!(res.is_finite());
    }
}
