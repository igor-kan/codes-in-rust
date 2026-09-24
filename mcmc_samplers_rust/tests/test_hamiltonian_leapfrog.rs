//! Tests for hamiltonian_leapfrog
#[path = "../src/hamiltonian_leapfrog.rs"]
mod hamiltonian_leapfrog;
use hamiltonian_leapfrog::*;

#[test]
fn test_hamiltonian_leapfrog_execution() {
    assert_eq!(leapfrog_p(1.0, 2.0, 0.1), 0.9);
}
