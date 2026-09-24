//! Tests for vasicek_yield_curve
#[path = "../src/vasicek_yield_curve.rs"]
mod vasicek_yield_curve;
use vasicek_yield_curve::*;

#[test]
fn test_vasicek_yield_curve_execution() {
    assert_eq!(zero_coupon_yield(1.0, 2.0), 0.0);
}
