pub fn compute_fourier_comp_term_28(x: f64) -> f64 {
    x.powi(28) / 28.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_28() {
        let res = compute_fourier_comp_term_28(1.0);
        assert!((res - (1.0 / 28.0)).abs() < 1e-7);
    }
}
