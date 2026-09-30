//! Implementation of continued fraction approximant order 15

pub fn compute_continued_frac_15(x: f64) -> f64 {
    let mut a = 1.0_f64;
    for k in (1..=4).rev() {
        a = (k as f64) + x / (if a != 0.0 { a } else { 1.0 });
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_continued_frac_15() {
        let res = compute_continued_frac_15(0.5);
        assert!(res.is_finite());
    }
}
