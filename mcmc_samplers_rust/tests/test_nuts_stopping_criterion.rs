//! Tests for nuts_stopping_criterion
#[path = "../src/nuts_stopping_criterion.rs"]
mod nuts_stopping_criterion;
use nuts_stopping_criterion::*;

#[test]
fn test_nuts_stopping_criterion_execution() {
    assert!(u_turn_detected(-0.1));
}
