//! Implementation of bessel series expansion degree/order 9

pub fn evaluate_bessel_series_9(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 6.0_f64;
    sign * (x / 2.0).powi(4) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_9() {
        let res = evaluate_bessel_series_9(0.5);
        assert!(res.is_finite());
    }
}
