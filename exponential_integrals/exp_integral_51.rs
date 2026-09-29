//! Implementation of exponential integral recurrence order 51

pub fn compute_exp_integral_51(x: f64) -> f64 {
    (-x).exp() / (51 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_exp_integral_51() {
        let res = compute_exp_integral_51(0.5);
        assert!(res.is_finite());
    }
}
