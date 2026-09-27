pub fn compute_fourier_comp_term_48(x: f64) -> f64 {
    x.powi(48) / 48.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_48() {
        let res = compute_fourier_comp_term_48(1.0);
        assert!((res - (1.0 / 48.0)).abs() < 1e-7);
    }
}
