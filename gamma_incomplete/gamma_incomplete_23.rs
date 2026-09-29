//! Implementation of incomplete gamma power series component order 23

pub fn compute_gamma_incomplete_23(x: f64) -> f64 {
    let s = 3.0_f64;
    x.powf(s + 3.0_f64) / (s + 3.0_f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_gamma_incomplete_23() {
        let res = compute_gamma_incomplete_23(0.5);
        assert!(res.is_finite());
    }
}
