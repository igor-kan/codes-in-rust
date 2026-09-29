//! Implementation of incomplete gamma power series component order 118

pub fn compute_gamma_incomplete_118(x: f64) -> f64 {
    let s = 7.0_f64;
    x.powf(s + 3.0_f64) / (s + 3.0_f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_gamma_incomplete_118() {
        let res = compute_gamma_incomplete_118(0.5);
        assert!(res.is_finite());
    }
}
