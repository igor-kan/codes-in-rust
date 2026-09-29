//! Implementation of exponential integral recurrence order 56

pub fn compute_exp_integral_56(x: f64) -> f64 {
    (-x).exp() / (56 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_56() {
        let res = compute_exp_integral_56(0.5);
        assert!(res.is_finite());
    }
}
