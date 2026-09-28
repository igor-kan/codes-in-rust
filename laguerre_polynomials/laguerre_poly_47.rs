//! Implementation of laguerre polynomial degree/order 47

pub fn evaluate_laguerre_poly_47(x: f64) -> f64 {
    if 47 == 0 { return 1.0; }
    if 47 == 1 { return 1.0 - x; }
    let mut p0 = 1.0;
    let mut p1 = 1.0 - x;
    for k in 1..47 {
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
    fn test_evaluate_laguerre_poly_47() {
        let res = evaluate_laguerre_poly_47(0.5);
        assert!(res.is_finite());
    }
}
