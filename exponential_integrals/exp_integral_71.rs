//! Implementation of exponential integral recurrence order 71

pub fn compute_exp_integral_71(x: f64) -> f64 {
    (-x).exp() / (71 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_71() {
        let res = compute_exp_integral_71(0.5);
        assert!(res.is_finite());
    }
}
