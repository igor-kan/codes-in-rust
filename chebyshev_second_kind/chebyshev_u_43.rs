//! Implementation of chebyshev second kind polynomial degree/order 43

pub fn evaluate_chebyshev_u_43(x: f64) -> f64 {
    if 43 == 0 { return 1.0; }
    if 43 == 1 { return 2.0 * x; }
    let mut p0 = 1.0;
    let mut p1 = 2.0 * x;
    for _ in 1..43 {
        let p_next = 2.0 * x * p1 - p0;
        p0 = p1;
        p1 = p_next;
    }
    p1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_chebyshev_u_43() {
        let res = evaluate_chebyshev_u_43(0.5);
        assert!(res.is_finite());
    }
}
