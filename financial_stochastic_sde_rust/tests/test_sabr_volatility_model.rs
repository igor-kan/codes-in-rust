//! Tests for sabr_volatility_model
#[path = "../src/sabr_volatility_model.rs"]
mod sabr_volatility_model;
use sabr_volatility_model::*;

#[test]
fn test_sabr_volatility_model_execution() {
    assert_eq!(sabr_backbone(4.0, 0.5), 2.0);
}
