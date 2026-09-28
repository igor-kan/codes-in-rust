//! Implementation of bessel series expansion degree/order 164

pub fn evaluate_bessel_series_164(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 3.722690410399437e+26_f64;
    sign * (x / 2.0).powi(29) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_164() {
        let res = evaluate_bessel_series_164(0.5);
        assert!(res.is_finite());
    }
}
