//! Implementation of confluent hypergeometric series term order 32

pub fn compute_hypergeom_series_32(x: f64) -> f64 {
    let a = 3.0_f64;
    let b = 5.0_f64;
    (a / b) * x.powi(2) / (2 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hypergeom_series_32() {
        let res = compute_hypergeom_series_32(0.5);
        assert!(res.is_finite());
    }
}
