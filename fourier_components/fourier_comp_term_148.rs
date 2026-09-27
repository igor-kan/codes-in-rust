pub fn compute_fourier_comp_term_148(x: f64) -> f64 {
    x.powi(148) / 148.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_148() {
        let res = compute_fourier_comp_term_148(1.0);
        assert!((res - (1.0 / 148.0)).abs() < 1e-7);
    }
}
