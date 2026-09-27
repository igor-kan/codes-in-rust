pub fn compute_fourier_comp_term_123(x: f64) -> f64 {
    x.powi(123) / 123.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_123() {
        let res = compute_fourier_comp_term_123(1.0);
        assert!((res - (1.0 / 123.0)).abs() < 1e-7);
    }
}
