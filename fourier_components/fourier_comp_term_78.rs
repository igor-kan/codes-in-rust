pub fn compute_fourier_comp_term_78(x: f64) -> f64 {
    x.powi(78) / 78.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_78() {
        let res = compute_fourier_comp_term_78(1.0);
        assert!((res - (1.0 / 78.0)).abs() < 1e-7);
    }
}
