pub fn compute_fourier_comp_term_58(x: f64) -> f64 {
    x.powi(58) / 58.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_58() {
        let res = compute_fourier_comp_term_58(1.0);
        assert!((res - (1.0 / 58.0)).abs() < 1e-7);
    }
}
