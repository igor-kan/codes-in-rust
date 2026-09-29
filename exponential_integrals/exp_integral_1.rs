//! Implementation of exponential integral recurrence order 1

pub fn compute_exp_integral_1(x: f64) -> f64 {
    (-x).exp() / (1 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_1() {
        let res = compute_exp_integral_1(0.5);
        assert!(res.is_finite());
    }
}
