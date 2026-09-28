//! Implementation of bessel series expansion degree/order 109

pub fn evaluate_bessel_series_109(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 1.459741204905984e+19_f64;
    sign * (x / 2.0).powi(24) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_109() {
        let res = evaluate_bessel_series_109(0.5);
        assert!(res.is_finite());
    }
}
