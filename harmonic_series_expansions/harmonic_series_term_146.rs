pub fn compute_harmonic_series_term_146(x: f64) -> f64 {
    x.powi(146) / 146.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_146() {
        let res = compute_harmonic_series_term_146(1.0);
        assert!((res - (1.0 / 146.0)).abs() < 1e-7);
    }
}
