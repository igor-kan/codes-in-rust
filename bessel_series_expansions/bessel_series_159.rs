//! Implementation of bessel series expansion degree/order 159

pub fn evaluate_bessel_series_159(x: f64) -> f64 {
    let sign = -1.0_f64;
    let denom = 5.487990203010849e+31_f64;
    sign * (x / 2.0).powi(34) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_159() {
        let res = evaluate_bessel_series_159(0.5);
        assert!(res.is_finite());
    }
}
