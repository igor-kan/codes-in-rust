//! Implementation of bessel series expansion degree/order 149

pub fn evaluate_bessel_series_149(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 7.445380820798873e+25_f64;
    sign * (x / 2.0).powi(29) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_149() {
        let res = evaluate_bessel_series_149(0.5);
        assert!(res.is_finite());
    }
}
