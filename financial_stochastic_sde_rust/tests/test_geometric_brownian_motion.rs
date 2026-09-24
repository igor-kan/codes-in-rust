//! Tests for geometric_brownian_motion
#[path = "../src/geometric_brownian_motion.rs"]
mod geometric_brownian_motion;
use geometric_brownian_motion::*;

#[test]
fn test_geometric_brownian_motion_execution() {
    assert!(gbm_path(100.0, 0.05, 0.0) > 100.0);
}
