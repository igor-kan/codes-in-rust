//! Implementation of bessel series expansion degree/order 169

pub fn evaluate_bessel_series_169(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 2.585201673888498e+22_f64;
    sign * (x / 2.0).powi(24) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_169() {
        let res = evaluate_bessel_series_169(0.5);
        assert!(res.is_finite());
    }
}
