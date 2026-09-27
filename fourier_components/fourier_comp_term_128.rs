pub fn compute_fourier_comp_term_128(x: f64) -> f64 {
    x.powi(128) / 128.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_128() {
        let res = compute_fourier_comp_term_128(1.0);
        assert!((res - (1.0 / 128.0)).abs() < 1e-7);
    }
}
