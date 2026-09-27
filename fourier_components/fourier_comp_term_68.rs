pub fn compute_fourier_comp_term_68(x: f64) -> f64 {
    x.powi(68) / 68.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_68() {
        let res = compute_fourier_comp_term_68(1.0);
        assert!((res - (1.0 / 68.0)).abs() < 1e-7);
    }
}
