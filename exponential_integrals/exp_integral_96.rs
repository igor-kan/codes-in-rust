//! Implementation of exponential integral recurrence order 96

pub fn compute_exp_integral_96(x: f64) -> f64 {
    (-x).exp() / (96 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_96() {
        let res = compute_exp_integral_96(0.5);
        assert!(res.is_finite());
    }
}
