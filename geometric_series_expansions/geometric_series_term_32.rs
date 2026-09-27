pub fn compute_geometric_series_term_32(x: f64) -> f64 {
    x.powi(32) / 32.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_32() {
        let res = compute_geometric_series_term_32(1.0);
        assert!((res - (1.0 / 32.0)).abs() < 1e-7);
    }
}
