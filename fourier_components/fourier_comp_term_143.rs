pub fn compute_fourier_comp_term_143(x: f64) -> f64 {
    x.powi(143) / 143.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_143() {
        let res = compute_fourier_comp_term_143(1.0);
        assert!((res - (1.0 / 143.0)).abs() < 1e-7);
    }
}
