//! Tests for affine_invariant_ensemble
#[path = "../src/affine_invariant_ensemble.rs"]
mod affine_invariant_ensemble;
use affine_invariant_ensemble::*;

#[test]
fn test_affine_invariant_ensemble_execution() {
    assert_eq!(stretch_move(2.0, 1.0, 2.0), 3.0);
}
