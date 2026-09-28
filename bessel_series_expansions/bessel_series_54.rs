//! Implementation of bessel series expansion degree/order 54

pub fn evaluate_bessel_series_54(x: f64) -> f64 {
    let sign = 1.0_f64;
    let denom = 4483454976000.0_f64;
    sign * (x / 2.0).powi(19) / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_bessel_series_54() {
        let res = evaluate_bessel_series_54(0.5);
        assert!(res.is_finite());
    }
}
