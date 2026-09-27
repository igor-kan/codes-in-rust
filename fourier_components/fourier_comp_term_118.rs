pub fn compute_fourier_comp_term_118(x: f64) -> f64 {
    x.powi(118) / 118.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_118() {
        let res = compute_fourier_comp_term_118(1.0);
        assert!((res - (1.0 / 118.0)).abs() < 1e-7);
    }
}
