pub fn compute_geometric_series_term_127(x: f64) -> f64 {
    x.powi(127) / 127.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_127() {
        let res = compute_geometric_series_term_127(1.0);
        assert!((res - (1.0 / 127.0)).abs() < 1e-7);
    }
}
