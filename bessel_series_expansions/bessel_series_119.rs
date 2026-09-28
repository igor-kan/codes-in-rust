//! Implementation of bessel series expansion degree/order 119

pub fn evaluate_bessel_series_119(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 5.664963667999142e+24_f64;
    sign * (x / 2.0).powi(29) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_119() {
        let res = evaluate_bessel_series_119(0.5);
        assert!(res.is_finite());
    }
}
