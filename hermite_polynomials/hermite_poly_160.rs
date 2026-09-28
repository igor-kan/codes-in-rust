//! Implementation of hermite polynomial degree/order 160

pub fn evaluate_hermite_poly_160(x: f64) -> f64 {
    if 160 == 0 { return 1.0; }
    if 160 == 1 { return 2.0 * x; }
    let mut p0 = 1.0;
    let mut p1 = 2.0 * x;
    for k in 1..160 {
        let p_next = 2.0 * x * p1 - 2.0 * (k as f64) * p0;
        p0 = p1;
        p1 = p_next;
    }
    p1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_hermite_poly_160() {
        let res = evaluate_hermite_poly_160(0.5);
        assert!(res.is_finite());
    }
}
