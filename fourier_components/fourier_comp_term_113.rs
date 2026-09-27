pub fn compute_fourier_comp_term_113(x: f64) -> f64 {
    x.powi(113) / 113.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_113() {
        let res = compute_fourier_comp_term_113(1.0);
        assert!((res - (1.0 / 113.0)).abs() < 1e-7);
    }
}
