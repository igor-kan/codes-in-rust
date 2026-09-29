//! Implementation of incomplete gamma power series component order 88

pub fn compute_gamma_incomplete_88(x: f64) -> f64 {
    let s = 5.0_f64;
    x.powf(s + 3.0_f64) / (s + 3.0_f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_gamma_incomplete_88() {
        let res = compute_gamma_incomplete_88(0.5);
        assert!(res.is_finite());
    }
}
