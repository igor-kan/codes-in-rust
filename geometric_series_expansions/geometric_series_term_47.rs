pub fn compute_geometric_series_term_47(x: f64) -> f64 {
    x.powi(47) / 47.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_47() {
        let res = compute_geometric_series_term_47(1.0);
        assert!((res - (1.0 / 47.0)).abs() < 1e-7);
    }
}
