pub fn compute_geometric_series_term_2(x: f64) -> f64 {
    x.powi(2) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_2() {
        let res = compute_geometric_series_term_2(1.0);
        assert!((res - (1.0 / 2.0)).abs() < 1e-7);
    }
}
