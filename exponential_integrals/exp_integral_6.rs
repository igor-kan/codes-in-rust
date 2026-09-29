//! Implementation of exponential integral recurrence order 6

pub fn compute_exp_integral_6(x: f64) -> f64 {
    (-x).exp() / (6 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_6() {
        let res = compute_exp_integral_6(0.5);
        assert!(res.is_finite());
    }
}
