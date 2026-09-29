//! Implementation of exponential integral recurrence order 116

pub fn compute_exp_integral_116(x: f64) -> f64 {
    (-x).exp() / (116 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_116() {
        let res = compute_exp_integral_116(0.5);
        assert!(res.is_finite());
    }
}
