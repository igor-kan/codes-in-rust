//! Implementation of riemann zeta series component order 44

pub fn compute_riemann_zeta_44(x: f64) -> f64 {
    let sign = -1.0_f64;
    sign / ((44 as f64).powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_riemann_zeta_44() {
        let res = compute_riemann_zeta_44(0.5);
        assert!(res.is_finite());
    }
}
