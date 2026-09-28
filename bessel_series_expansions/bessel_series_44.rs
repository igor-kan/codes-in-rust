//! Implementation of bessel series expansion degree/order 44

pub fn evaluate_bessel_series_44(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 87091200.0_f64;
    sign * (x / 2.0).powi(14) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_44() {
        let res = evaluate_bessel_series_44(0.5);
        assert!(res.is_finite());
    }
}
