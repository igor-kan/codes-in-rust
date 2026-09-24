//! Tests for cliquet_local_cap
#[path = "../src/cliquet_local_cap.rs"]
mod cliquet_local_cap;
use cliquet_local_cap::*;

#[test]
fn test_cliquet_local_cap_execution() {
    assert_eq!(capped_return(0.08, 0.05, -0.02), 0.05);
}
