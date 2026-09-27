pub fn compute_fourier_comp_term_138(x: f64) -> f64 {
    x.powi(138) / 138.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_138() {
        let res = compute_fourier_comp_term_138(1.0);
        assert!((res - (1.0 / 138.0)).abs() < 1e-7);
    }
}
