pub fn compute_fourier_comp_term_33(x: f64) -> f64 {
    x.powi(33) / 33.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_33() {
        let res = compute_fourier_comp_term_33(1.0);
        assert!((res - (1.0 / 33.0)).abs() < 1e-7);
    }
}
