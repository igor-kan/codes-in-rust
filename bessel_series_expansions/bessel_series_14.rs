//! Implementation of bessel series expansion degree/order 14

pub fn evaluate_bessel_series_14(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 29030400.0_f64;
    sign * (x / 2.0).powi(14) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_14() {
        let res = evaluate_bessel_series_14(0.5);
        assert!(res.is_finite());
    }
}
