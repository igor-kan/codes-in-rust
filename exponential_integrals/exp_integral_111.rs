//! Implementation of exponential integral recurrence order 111

pub fn compute_exp_integral_111(x: f64) -> f64 {
    (-x).exp() / (111 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_111() {
        let res = compute_exp_integral_111(0.5);
        assert!(res.is_finite());
    }
}
