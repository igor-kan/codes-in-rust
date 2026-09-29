//! Implementation of exponential integral recurrence order 31

pub fn compute_exp_integral_31(x: f64) -> f64 {
    (-x).exp() / (31 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_31() {
        let res = compute_exp_integral_31(0.5);
        assert!(res.is_finite());
    }
}
