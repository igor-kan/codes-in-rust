pub fn compute_harmonic_series_term_51(x: f64) -> f64 {
    x.powi(51) / 51.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_51() {
        let res = compute_harmonic_series_term_51(1.0);
        assert!((res - (1.0 / 51.0)).abs() < 1e-7);
    }
}
