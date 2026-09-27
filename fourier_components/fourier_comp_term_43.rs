pub fn compute_fourier_comp_term_43(x: f64) -> f64 {
    x.powi(43) / 43.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_43() {
        let res = compute_fourier_comp_term_43(1.0);
        assert!((res - (1.0 / 43.0)).abs() < 1e-7);
    }
}
