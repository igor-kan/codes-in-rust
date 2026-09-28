//! Implementation of bessel series expansion degree/order 154

pub fn evaluate_bessel_series_154(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 2.2480014555552154e+21_f64;
    sign * (x / 2.0).powi(24) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_154() {
        let res = evaluate_bessel_series_154(0.5);
        assert!(res.is_finite());
    }
}
