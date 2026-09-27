pub fn compute_geometric_series_term_137(x: f64) -> f64 {
    x.powi(137) / 137.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_137() {
        let res = compute_geometric_series_term_137(1.0);
        assert!((res - (1.0 / 137.0)).abs() < 1e-7);
    }
}
