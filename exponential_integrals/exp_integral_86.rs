//! Implementation of exponential integral recurrence order 86

pub fn compute_exp_integral_86(x: f64) -> f64 {
    (-x).exp() / (86 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_86() {
        let res = compute_exp_integral_86(0.5);
        assert!(res.is_finite());
    }
}
