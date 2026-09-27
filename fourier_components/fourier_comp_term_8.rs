pub fn compute_fourier_comp_term_8(x: f64) -> f64 {
    x.powi(8) / 8.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_8() {
        let res = compute_fourier_comp_term_8(1.0);
        assert!((res - (1.0 / 8.0)).abs() < 1e-7);
    }
}
