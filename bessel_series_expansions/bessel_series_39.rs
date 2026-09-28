//! Implementation of bessel series expansion degree/order 39

pub fn evaluate_bessel_series_39(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 2414168064000.0_f64;
    sign * (x / 2.0).powi(19) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_39() {
        let res = evaluate_bessel_series_39(0.5);
        assert!(res.is_finite());
    }
}
