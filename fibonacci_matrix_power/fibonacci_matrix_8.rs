//! Implementation of fibonacci matrix power recurrence order 8

pub fn compute_fibonacci_matrix_8(x: f64) -> f64 {
    let mut f0 = 1.0_f64;
    let mut f1 = 1.0_f64;
    for _ in 0..9 {
        let next = f0 + f1 * x * 0.1;
        f0 = f1;
        f1 = next;
    }
    f1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fibonacci_matrix_8() {
        let res = compute_fibonacci_matrix_8(0.5);
        assert!(res.is_finite());
    }
}
