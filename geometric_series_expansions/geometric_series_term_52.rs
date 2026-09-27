pub fn compute_geometric_series_term_52(x: f64) -> f64 {
    x.powi(52) / 52.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_52() {
        let res = compute_geometric_series_term_52(1.0);
        assert!((res - (1.0 / 52.0)).abs() < 1e-7);
    }
}
