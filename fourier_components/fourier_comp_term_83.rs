pub fn compute_fourier_comp_term_83(x: f64) -> f64 {
    x.powi(83) / 83.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_83() {
        let res = compute_fourier_comp_term_83(1.0);
        assert!((res - (1.0 / 83.0)).abs() < 1e-7);
    }
}
