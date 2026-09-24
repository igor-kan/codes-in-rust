//! Tests for garman_kohlhagen_fx
#[path = "../src/garman_kohlhagen_fx.rs"]
mod garman_kohlhagen_fx;
use garman_kohlhagen_fx::*;

#[test]
fn test_garman_kohlhagen_fx_execution() {
    assert_eq!(fx_drift(0.05, 0.02), 0.03);
}
