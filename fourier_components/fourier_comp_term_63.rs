pub fn compute_fourier_comp_term_63(x: f64) -> f64 {
    x.powi(63) / 63.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_63() {
        let res = compute_fourier_comp_term_63(1.0);
        assert!((res - (1.0 / 63.0)).abs() < 1e-7);
    }
}
