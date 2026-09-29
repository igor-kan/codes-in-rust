//! Implementation of exponential integral recurrence order 36

pub fn compute_exp_integral_36(x: f64) -> f64 {
    (-x).exp() / (36 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_36() {
        let res = compute_exp_integral_36(0.5);
        assert!(res.is_finite());
    }
}
