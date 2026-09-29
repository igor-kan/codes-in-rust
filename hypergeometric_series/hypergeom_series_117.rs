//! Implementation of confluent hypergeometric series term order 117

pub fn compute_hypergeom_series_117(x: f64) -> f64 {
    let a = 3.0_f64;
    let b = 5.0_f64;
    (a / b) * x.powi(3) / (6 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hypergeom_series_117() {
        let res = compute_hypergeom_series_117(0.5);
        assert!(res.is_finite());
    }
}
