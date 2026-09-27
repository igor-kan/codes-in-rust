pub fn compute_fourier_comp_term_53(x: f64) -> f64 {
    x.powi(53) / 53.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_53() {
        let res = compute_fourier_comp_term_53(1.0);
        assert!((res - (1.0 / 53.0)).abs() < 1e-7);
    }
}
