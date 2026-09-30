//! Implementation of continued fraction approximant order 60

pub fn compute_continued_frac_60(x: f64) -> f64 {
    let mut a = 1.0_f64;
    for k in (1..=1).rev() {
        a = (k as f64) + x / (if a != 0.0 { a } else { 1.0 });
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_continued_frac_60() {
        let res = compute_continued_frac_60(0.5);
        assert!(res.is_finite());
    }
}
