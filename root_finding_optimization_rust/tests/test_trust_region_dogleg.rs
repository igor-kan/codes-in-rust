//! Tests for trust_region_dogleg
#[path = "../src/trust_region_dogleg.rs"]
mod trust_region_dogleg;
use trust_region_dogleg::*;

#[test]
fn test_trust_region_dogleg_execution() {
    assert_eq!(cauchy_point(4.0, 2.0), -2.0);
}
