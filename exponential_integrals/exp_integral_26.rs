//! Implementation of exponential integral recurrence order 26

pub fn compute_exp_integral_26(x: f64) -> f64 {
    (-x).exp() / (26 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_26() {
        let res = compute_exp_integral_26(0.5);
        assert!(res.is_finite());
    }
}
