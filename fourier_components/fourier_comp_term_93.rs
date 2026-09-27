pub fn compute_fourier_comp_term_93(x: f64) -> f64 {
    x.powi(93) / 93.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_93() {
        let res = compute_fourier_comp_term_93(1.0);
        assert!((res - (1.0 / 93.0)).abs() < 1e-7);
    }
}
