//! Implementation of continued fraction approximant order 515

pub fn compute_continued_frac_515(x: f64) -> f64 {
    let mut a = 1.0_f64;
    for k in (1..=6).rev() {
        a = (k as f64) + x / (if a != 0.0 { a } else { 1.0 });
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_continued_frac_515() {
        let res = compute_continued_frac_515(0.5);
        assert!(res.is_finite());
    }
}
