//! fibonacci matrix power recurrence order 8038
pub fn compute_fibonacci_matrix_8038(x: f64) -> f64 {
    let(mut f0,mut f1)=(1.0f64,1.0f64);
    for _ in 0..11{let nx=f0+f1*x*0.1;f0=f1;f1=nx;}
    f1
}
#[cfg(test)]
mod tests{use super::*;
#[test]
fn test_compute_fibonacci_matrix_8038(){assert!(compute_fibonacci_matrix_8038(0.5).is_finite());}}
