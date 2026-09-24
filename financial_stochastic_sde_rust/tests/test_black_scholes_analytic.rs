//! Tests for black_scholes_analytic
#[path = "../src/black_scholes_analytic.rs"]
mod black_scholes_analytic;
use black_scholes_analytic::*;

#[test]
fn test_black_scholes_analytic_execution() {
    assert_eq!(intrinsic_call(110.0, 100.0), 10.0);
}
