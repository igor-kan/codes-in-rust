//! continued fraction approximant order 16270
pub fn compute_continued_frac_16270(x: f64) -> f64 {
    let mut a=1.0f64;
    for k in (1..=5).rev(){a=(k as f64)+x/(if a!=0.0{a}else{1.0});}
    a
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_continued_frac_16270(){assert!(compute_continued_frac_16270(0.5).is_finite());}}
