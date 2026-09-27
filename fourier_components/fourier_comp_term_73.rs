pub fn compute_fourier_comp_term_73(x: f64) -> f64 {
    x.powi(73) / 73.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_73() {
        let res = compute_fourier_comp_term_73(1.0);
        assert!((res - (1.0 / 73.0)).abs() < 1e-7);
    }
}
