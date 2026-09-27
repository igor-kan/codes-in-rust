pub fn compute_fourier_comp_term_98(x: f64) -> f64 {
    x.powi(98) / 98.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_98() {
        let res = compute_fourier_comp_term_98(1.0);
        assert!((res - (1.0 / 98.0)).abs() < 1e-7);
    }
}
