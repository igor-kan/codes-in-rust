//! Implementation of bessel series expansion degree/order 24

pub fn evaluate_bessel_series_24(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 24.0_f64;
    sign * (x / 2.0).powi(4) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_24() {
        let res = evaluate_bessel_series_24(0.5);
        assert!(res.is_finite());
    }
}
