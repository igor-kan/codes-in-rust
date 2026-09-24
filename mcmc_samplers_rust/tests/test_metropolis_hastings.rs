//! Tests for metropolis_hastings
#[path = "../src/metropolis_hastings.rs"]
mod metropolis_hastings;
use metropolis_hastings::*;

#[test]
fn test_metropolis_hastings_execution() {
    assert_eq!(accept_prob(1.0, 0.5), 0.5); assert_eq!(accept_prob(0.5, 1.0), 1.0);
}
