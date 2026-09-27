pub fn compute_geometric_series_term_142(x: f64) -> f64 {
    x.powi(142) / 142.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_142() {
        let res = compute_geometric_series_term_142(1.0);
        assert!((res - (1.0 / 142.0)).abs() < 1e-7);
    }
}
