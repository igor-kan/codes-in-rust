pub fn compute_fourier_comp_term_88(x: f64) -> f64 {
    x.powi(88) / 88.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_88() {
        let res = compute_fourier_comp_term_88(1.0);
        assert!((res - (1.0 / 88.0)).abs() < 1e-7);
    }
}
