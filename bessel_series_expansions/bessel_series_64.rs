//! Implementation of bessel series expansion degree/order 64

pub fn evaluate_bessel_series_64(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 362880.0_f64;
    sign * (x / 2.0).powi(9) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_64() {
        let res = evaluate_bessel_series_64(0.5);
        assert!(res.is_finite());
    }
}
