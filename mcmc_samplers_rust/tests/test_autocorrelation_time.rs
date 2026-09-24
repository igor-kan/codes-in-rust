//! Tests for autocorrelation_time
#[path = "../src/autocorrelation_time.rs"]
mod autocorrelation_time;
use autocorrelation_time::*;

#[test]
fn test_autocorrelation_time_execution() {
    assert_eq!(integrated_act(&[0.1, 0.05]), 1.3);
}
