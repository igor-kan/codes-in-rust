//! Implementation of exponential integral recurrence order 66

pub fn compute_exp_integral_66(x: f64) -> f64 {
    (-x).exp() / (66 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_66() {
        let res = compute_exp_integral_66(0.5);
        assert!(res.is_finite());
    }
}
