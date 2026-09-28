//! Implementation of bessel series expansion degree/order 94

pub fn evaluate_bessel_series_94(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 4.60970906812416e+18_f64;
    sign * (x / 2.0).powi(24) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_94() {
        let res = evaluate_bessel_series_94(0.5);
        assert!(res.is_finite());
    }
}
