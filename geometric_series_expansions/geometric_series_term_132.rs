pub fn compute_geometric_series_term_132(x: f64) -> f64 {
    x.powi(132) / 132.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_132() {
        let res = compute_geometric_series_term_132(1.0);
        assert!((res - (1.0 / 132.0)).abs() < 1e-7);
    }
}
