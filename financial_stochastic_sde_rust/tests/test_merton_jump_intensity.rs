//! Tests for merton_jump_intensity
#[path = "../src/merton_jump_intensity.rs"]
mod merton_jump_intensity;
use merton_jump_intensity::*;

#[test]
fn test_merton_jump_intensity_execution() {
    assert_eq!(jump_probability(0.5, 0.1), 0.05);
}
