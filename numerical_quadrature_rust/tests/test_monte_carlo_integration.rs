//! Tests for monte_carlo_integration
#[path = "../src/monte_carlo_integration.rs"]
mod monte_carlo_integration;
use monte_carlo_integration::*;

#[test]
fn test_monte_carlo_integration_execution() {
    assert_eq!(mc_estimate(200.0, 100, 2.0), 4.0);
}
