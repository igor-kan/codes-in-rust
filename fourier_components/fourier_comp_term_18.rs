pub fn compute_fourier_comp_term_18(x: f64) -> f64 {
    x.powi(18) / 18.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_18() {
        let res = compute_fourier_comp_term_18(1.0);
        assert!((res - (1.0 / 18.0)).abs() < 1e-7);
    }
}
