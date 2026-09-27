pub fn compute_geometric_series_term_92(x: f64) -> f64 {
    x.powi(92) / 92.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_92() {
        let res = compute_geometric_series_term_92(1.0);
        assert!((res - (1.0 / 92.0)).abs() < 1e-7);
    }
}
