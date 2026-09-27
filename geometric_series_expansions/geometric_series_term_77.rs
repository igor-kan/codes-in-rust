pub fn compute_geometric_series_term_77(x: f64) -> f64 {
    x.powi(77) / 77.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_77() {
        let res = compute_geometric_series_term_77(1.0);
        assert!((res - (1.0 / 77.0)).abs() < 1e-7);
    }
}
