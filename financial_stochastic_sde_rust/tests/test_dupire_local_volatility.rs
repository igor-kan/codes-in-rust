//! Tests for dupire_local_volatility
#[path = "../src/dupire_local_volatility.rs"]
mod dupire_local_volatility;
use dupire_local_volatility::*;

#[test]
fn test_dupire_local_volatility_execution() {
    assert_eq!(dupire_denominator(10.0, 2.0), 100.0);
}
