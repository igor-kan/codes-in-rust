//! Tests for boole_rule_quadrature
#[path = "../src/boole_rule_quadrature.rs"]
mod boole_rule_quadrature;
use boole_rule_quadrature::*;

#[test]
fn test_boole_rule_quadrature_execution() {
    let f = [1.0, 1.0, 1.0, 1.0, 1.0]; assert!((boole_quad(&f, 1.0) - 4.0).abs() < 1e-7);
}
