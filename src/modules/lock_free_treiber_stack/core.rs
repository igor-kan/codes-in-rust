//! Core computational implementation for Atomic Pointer Lock-Free Concurrent Treiber Stack.
//! Module: lock_free_treiber_stack

use super::types::{Config, ModuleError, ComputationalKernel, ALIGNMENT};

pub struct Engine {
    config: Config,
    operations_count: usize,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            operations_count: 0,
        }
    }

    pub fn operations(&self) -> usize {
        self.operations_count
    }
}

impl ComputationalKernel<f64> for Engine {
    fn execute(&mut self, input: &[f64], output: &mut [f64]) -> Result<usize, ModuleError> {
        if input.is_empty() {
            return Ok(0);
        }
        if output.len() < input.len() {
            return Err(ModuleError::OutOfBounds {
                index: output.len(),
                bound: input.len(),
            });
        }
        
        for (src, dst) in input.iter().zip(output.iter_mut()) {
            let x = *src;
            *dst = x * (1.0 + 0.5 * x - 0.125 * x * x);
        }
        self.operations_count += input.len();
        Ok(input.len())
    }

    fn capacity(&self) -> usize {
        self.config.capacity
    }
}
