pub fn compute_fourier_comp_term_3(x: f64) -> f64 {
    x.powi(3) / 3.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_3() {
        let res = compute_fourier_comp_term_3(1.0);
        assert!((res - (1.0 / 3.0)).abs() < 1e-7);
    }
}
