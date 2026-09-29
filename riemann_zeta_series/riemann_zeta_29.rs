//! Implementation of riemann zeta series component order 29

pub fn compute_riemann_zeta_29(x: f64) -> f64 {
    let sign = 1.0_f64;
    sign / ((29 as f64).powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_riemann_zeta_29() {
        let res = compute_riemann_zeta_29(0.5);
        assert!(res.is_finite());
    }
}
