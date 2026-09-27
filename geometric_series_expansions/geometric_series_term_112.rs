pub fn compute_geometric_series_term_112(x: f64) -> f64 {
    x.powi(112) / 112.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_112() {
        let res = compute_geometric_series_term_112(1.0);
        assert!((res - (1.0 / 112.0)).abs() < 1e-7);
    }
}
