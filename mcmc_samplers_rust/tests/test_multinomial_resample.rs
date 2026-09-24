//! Tests for multinomial_resample
#[path = "../src/multinomial_resample.rs"]
mod multinomial_resample;
use multinomial_resample::*;

#[test]
fn test_multinomial_resample_execution() {
    let c = cumulative_sum(&[0.2, 0.3, 0.5]); assert_eq!(c[2], 1.0);
}
