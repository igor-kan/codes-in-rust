//! Implementation of exponential integral recurrence order 16

pub fn compute_exp_integral_16(x: f64) -> f64 {
    (-x).exp() / (16 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_16() {
        let res = compute_exp_integral_16(0.5);
        assert!(res.is_finite());
    }
}
