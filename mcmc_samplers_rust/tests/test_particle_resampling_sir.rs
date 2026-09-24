//! Tests for particle_resampling_sir
#[path = "../src/particle_resampling_sir.rs"]
mod particle_resampling_sir;
use particle_resampling_sir::*;

#[test]
fn test_particle_resampling_sir_execution() {
    assert_eq!(effective_particles(0.25), 4.0);
}
