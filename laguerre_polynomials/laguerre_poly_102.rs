//! Implementation of laguerre polynomial degree/order 102

pub fn evaluate_laguerre_poly_102(x: f64) -> f64 {
    if 102 == 0 { return 1.0; }
    if 102 == 1 { return 1.0 - x; }
    let mut p0 = 1.0;
    let mut p1 = 1.0 - x;
    for k in 1..102 {
        let k_f = k as f64;
        let p_next = ((2.0 * k_f + 1.0 - x) * p1 - k_f * p0) / (k_f + 1.0);
        p0 = p1;
        p1 = p_next;
    }
    p1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_laguerre_poly_102() {
        let res = evaluate_laguerre_poly_102(0.5);
        assert!(res.is_finite());
    }
}
