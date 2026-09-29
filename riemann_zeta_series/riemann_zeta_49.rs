//! Implementation of riemann zeta series component order 49

pub fn compute_riemann_zeta_49(x: f64) -> f64 {
    let sign = 1.0_f64;
    sign / ((49 as f64).powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_riemann_zeta_49() {
        let res = compute_riemann_zeta_49(0.5);
        assert!(res.is_finite());
    }
}
