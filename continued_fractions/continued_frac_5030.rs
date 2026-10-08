//! continued fraction approximant order 5030
pub fn compute_continued_frac_5030(x: f64) -> f64 {
    let mut a=1.0f64;
    for k in (1..=3).rev(){a=(k as f64)+x/(if a!=0.0{a}else{1.0});}
    a
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_continued_frac_5030(){assert!(compute_continued_frac_5030(0.5).is_finite());}}
