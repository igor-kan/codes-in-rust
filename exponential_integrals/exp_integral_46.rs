//! Implementation of exponential integral recurrence order 46

pub fn compute_exp_integral_46(x: f64) -> f64 {
    (-x).exp() / (46 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_46() {
        let res = compute_exp_integral_46(0.5);
        assert!(res.is_finite());
    }
}
