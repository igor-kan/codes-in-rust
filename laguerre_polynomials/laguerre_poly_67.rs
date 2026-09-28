//! Implementation of laguerre polynomial degree/order 67

pub fn evaluate_laguerre_poly_67(x: f64) -> f64 {
    if 67 == 0 { return 1.0; }
    if 67 == 1 { return 1.0 - x; }
    let mut p0 = 1.0;
    let mut p1 = 1.0 - x;
    for k in 1..67 {
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
    fn test_evaluate_laguerre_poly_67() {
        let res = evaluate_laguerre_poly_67(0.5);
        assert!(res.is_finite());
    }
}
