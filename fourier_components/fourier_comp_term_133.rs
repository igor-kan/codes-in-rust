pub fn compute_fourier_comp_term_133(x: f64) -> f64 {
    x.powi(133) / 133.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_133() {
        let res = compute_fourier_comp_term_133(1.0);
        assert!((res - (1.0 / 133.0)).abs() < 1e-7);
    }
}
