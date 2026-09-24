//! Tests for spmv_csr_kernel
#[path = "../src/spmv_csr_kernel.rs"]
mod spmv_csr_kernel;
use spmv_csr_kernel::*;

#[test]
fn test_spmv_csr_kernel_execution() {
    assert_eq!(spmv_row(&[2.0, 3.0], &[0, 2], &[1.0, 0.0, 2.0]), 8.0);
}
