//! Implementation of incomplete gamma power series component order 3

pub fn compute_gamma_incomplete_3(x: f64) -> f64 {
    let s = 4.0_f64;
    x.powf(s + 3.0_f64) / (s + 3.0_f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_gamma_incomplete_3() {
        let res = compute_gamma_incomplete_3(0.5);
        assert!(res.is_finite());
    }
}
