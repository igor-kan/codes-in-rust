//! Implementation of legendre polynomial degree/order 101

pub fn evaluate_legendre_poly_101(x: f64) -> f64 {
    if 101 == 0 { return 1.0; }
    if 101 == 1 { return x; }
    let mut p0 = 1.0;
    let mut p1 = x;
    for k in 2..=101 {
        let k_f = k as f64;
        let p_next = ((2.0 * k_f - 1.0) * x * p1 - (k_f - 1.0) * p0) / k_f;
        p0 = p1;
        p1 = p_next;
    }
    p1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_legendre_poly_101() {
        let res = evaluate_legendre_poly_101(0.5);
        assert!(res.is_finite());
    }
}
