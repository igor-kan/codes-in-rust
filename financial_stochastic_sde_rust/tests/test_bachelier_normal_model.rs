//! Tests for bachelier_normal_model
#[path = "../src/bachelier_normal_model.rs"]
mod bachelier_normal_model;
use bachelier_normal_model::*;

#[test]
fn test_bachelier_normal_model_execution() {
    assert_eq!(bachelier_call_intrinsic(95.0, 100.0), 0.0);
}
