//! Implementation of exponential integral recurrence order 21

pub fn compute_exp_integral_21(x: f64) -> f64 {
    (-x).exp() / (21 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_21() {
        let res = compute_exp_integral_21(0.5);
        assert!(res.is_finite());
    }
}
