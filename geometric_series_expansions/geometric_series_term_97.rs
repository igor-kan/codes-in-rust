pub fn compute_geometric_series_term_97(x: f64) -> f64 {
    x.powi(97) / 97.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_97() {
        let res = compute_geometric_series_term_97(1.0);
        assert!((res - (1.0 / 97.0)).abs() < 1e-7);
    }
}
