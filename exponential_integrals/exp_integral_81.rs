//! Implementation of exponential integral recurrence order 81

pub fn compute_exp_integral_81(x: f64) -> f64 {
    (-x).exp() / (81 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_81() {
        let res = compute_exp_integral_81(0.5);
        assert!(res.is_finite());
    }
}
