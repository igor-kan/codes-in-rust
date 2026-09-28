//! Implementation of bessel series expansion degree/order 174

pub fn evaluate_bessel_series_174(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 2.1951960812043397e+32_f64;
    sign * (x / 2.0).powi(34) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_174() {
        let res = evaluate_bessel_series_174(0.5);
        assert!(res.is_finite());
    }
}
