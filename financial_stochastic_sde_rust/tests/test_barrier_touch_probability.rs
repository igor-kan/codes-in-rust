//! Tests for barrier_touch_probability
#[path = "../src/barrier_touch_probability.rs"]
mod barrier_touch_probability;
use barrier_touch_probability::*;

#[test]
fn test_barrier_touch_probability_execution() {
    assert!(touched_barrier(105.0, 100.0));
}
