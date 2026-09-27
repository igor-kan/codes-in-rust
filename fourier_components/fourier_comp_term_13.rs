pub fn compute_fourier_comp_term_13(x: f64) -> f64 {
    x.powi(13) / 13.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_13() {
        let res = compute_fourier_comp_term_13(1.0);
        assert!((res - (1.0 / 13.0)).abs() < 1e-7);
    }
}
