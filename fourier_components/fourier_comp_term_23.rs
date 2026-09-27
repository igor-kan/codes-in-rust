pub fn compute_fourier_comp_term_23(x: f64) -> f64 {
    x.powi(23) / 23.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_23() {
        let res = compute_fourier_comp_term_23(1.0);
        assert!((res - (1.0 / 23.0)).abs() < 1e-7);
    }
}
