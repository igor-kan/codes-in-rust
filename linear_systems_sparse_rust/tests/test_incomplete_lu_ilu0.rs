//! Tests for incomplete_lu_ilu0
#[path = "../src/incomplete_lu_ilu0.rs"]
mod incomplete_lu_ilu0;
use incomplete_lu_ilu0::*;

#[test]
fn test_incomplete_lu_ilu0_execution() {
    assert_eq!(ilu0_diag(5.0, 2.0), 3.0);
}
