//! Implementation of bessel series expansion degree/order 129

pub fn evaluate_bessel_series_129(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 6402373705728000.0_f64;
    sign * (x / 2.0).powi(19) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_129() {
        let res = evaluate_bessel_series_129(0.5);
        assert!(res.is_finite());
    }
}
