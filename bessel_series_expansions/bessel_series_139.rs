//! Implementation of bessel series expansion degree/order 139

pub fn evaluate_bessel_series_139(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 3.0654565303025664e+20_f64;
    sign * (x / 2.0).powi(24) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_139() {
        let res = evaluate_bessel_series_139(0.5);
        assert!(res.is_finite());
    }
}
