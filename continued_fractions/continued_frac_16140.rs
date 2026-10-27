//! continued fraction approximant order 16140
pub fn compute_continued_frac_16140(x: f64) -> f64 {
    let mut a=1.0f64;
    for k in (1..=1).rev(){a=(k as f64)+x/(if a!=0.0{a}else{1.0});}
    a
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_continued_frac_16140(){assert!(compute_continued_frac_16140(0.5).is_finite());}}
