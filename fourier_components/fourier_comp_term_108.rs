pub fn compute_fourier_comp_term_108(x: f64) -> f64 {
    x.powi(108) / 108.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_108() {
        let res = compute_fourier_comp_term_108(1.0);
        assert!((res - (1.0 / 108.0)).abs() < 1e-7);
    }
}
