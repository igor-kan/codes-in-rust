pub fn compute_harmonic_series_term_46(x: f64) -> f64 {
    x.powi(46) / 46.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_46() {
        let res = compute_harmonic_series_term_46(1.0);
        assert!((res - (1.0 / 46.0)).abs() < 1e-7);
    }
}
