//! Implementation of riemann zeta series component order 104

pub fn compute_riemann_zeta_104(x: f64) -> f64 {
    let sign = -1.0_f64;
    sign / ((104 as f64).powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_riemann_zeta_104() {
        let res = compute_riemann_zeta_104(0.5);
        assert!(res.is_finite());
    }
}
