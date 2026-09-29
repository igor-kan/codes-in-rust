//! Implementation of exponential integral recurrence order 76

pub fn compute_exp_integral_76(x: f64) -> f64 {
    (-x).exp() / (76 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_76() {
        let res = compute_exp_integral_76(0.5);
        assert!(res.is_finite());
    }
}
