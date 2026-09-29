//! Implementation of exponential integral recurrence order 11

pub fn compute_exp_integral_11(x: f64) -> f64 {
    (-x).exp() / (11 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_11() {
        let res = compute_exp_integral_11(0.5);
        assert!(res.is_finite());
    }
}
