pub fn compute_harmonic_series_term_136(x: f64) -> f64 {
    x.powi(136) / 136.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_136() {
        let res = compute_harmonic_series_term_136(1.0);
        assert!((res - (1.0 / 136.0)).abs() < 1e-7);
    }
}
