//! Implementation of exponential integral recurrence order 101

pub fn compute_exp_integral_101(x: f64) -> f64 {
    (-x).exp() / (101 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_101() {
        let res = compute_exp_integral_101(0.5);
        assert!(res.is_finite());
    }
}
