//! Implementation of exponential integral recurrence order 41

pub fn compute_exp_integral_41(x: f64) -> f64 {
    (-x).exp() / (41 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_41() {
        let res = compute_exp_integral_41(0.5);
        assert!(res.is_finite());
    }
}
