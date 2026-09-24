//! Tests for mala_drift_step
#[path = "../src/mala_drift_step.rs"]
mod mala_drift_step;
use mala_drift_step::*;

#[test]
fn test_mala_drift_step_execution() {
    assert_eq!(mala_mean(1.0, -1.0, 0.1), 0.9);
}
