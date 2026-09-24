//! Tests for simulated_annealing_cool
#[path = "../src/simulated_annealing_cool.rs"]
mod simulated_annealing_cool;
use simulated_annealing_cool::*;

#[test]
fn test_simulated_annealing_cool_execution() {
    assert!(boltzmann_accept(1.0, 2.0) < 1.0);
}
