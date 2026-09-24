//! Tests for hull_white_short_rate
#[path = "../src/hull_white_short_rate.rs"]
mod hull_white_short_rate;
use hull_white_short_rate::*;

#[test]
fn test_hull_white_short_rate_execution() {
    assert!(hw_variance(0.01, 0.1, 1.0) > 0.0);
}
