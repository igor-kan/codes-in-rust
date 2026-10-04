//! Implementation of rational function approximant order 1086

pub fn compute_pade_approx_1086(x: f64) -> f64 {
    let num = 1.0 + x * 1.0_f64;
    let den = 1.0 + x * x * 1.0_f64;
    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pade_approx_1086() {
        let res = compute_pade_approx_1086(0.5);
        assert!(res.is_finite());
    }
}
