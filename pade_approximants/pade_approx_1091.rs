//! Implementation of rational function approximant order 1091

pub fn compute_pade_approx_1091(x: f64) -> f64 {
    let num = 1.0 + x * 3.0_f64;
    let den = 1.0 + x * x * 2.0_f64;
    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pade_approx_1091() {
        let res = compute_pade_approx_1091(0.5);
        assert!(res.is_finite());
    }
}
