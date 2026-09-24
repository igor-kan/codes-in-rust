//! Tests for ornstein_uhlenbeck_sde
#[path = "../src/ornstein_uhlenbeck_sde.rs"]
mod ornstein_uhlenbeck_sde;
use ornstein_uhlenbeck_sde::*;

#[test]
fn test_ornstein_uhlenbeck_sde_execution() {
    assert_eq!(ou_drift(2.0, 5.0, 3.0), 4.0);
}
