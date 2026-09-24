//! Tests for heston_correlation
#[path = "../src/heston_correlation.rs"]
mod heston_correlation;
use heston_correlation::*;

#[test]
fn test_heston_correlation_execution() {
    assert_eq!(correlated_brownian(1.0, 0.0, 1.0), 1.0);
}
