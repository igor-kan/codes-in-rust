pub fn compute_fourier_comp_term_103(x: f64) -> f64 {
    x.powi(103) / 103.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_103() {
        let res = compute_fourier_comp_term_103(1.0);
        assert!((res - (1.0 / 103.0)).abs() < 1e-7);
    }
}
