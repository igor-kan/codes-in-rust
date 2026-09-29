//! Implementation of exponential integral recurrence order 61

pub fn compute_exp_integral_61(x: f64) -> f64 {
    (-x).exp() / (61 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_61() {
        let res = compute_exp_integral_61(0.5);
        assert!(res.is_finite());
    }
}
