//! Core type definitions and interfaces for Scoped Bump Allocation Arena for Zero-Thrash Workflows.
//! Module: arena_bump_allocator

use std::error::Error;
use std::fmt;

pub const ALIGNMENT: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub capacity: usize,
    pub tolerance: f64,
    pub flags: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            capacity: 1024,
            tolerance: 1e-12,
            flags: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleError {
    NullPointer,
    InvalidParameter(&'static str),
    OutOfBounds { index: usize, bound: usize },
    AllocationFailed,
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullPointer => write!(f, "Encountered null pointer or empty reference"),
            Self::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            Self::OutOfBounds { index, bound } => write!(f, "Index {} out of bounds {}", index, bound),
            Self::AllocationFailed => write!(f, "Memory allocation or alignment failed"),
        }
    }
}

impl Error for ModuleError {}

pub trait ComputationalKernel<T> {
    fn execute(&mut self, input: &[T], output: &mut [T]) -> Result<usize, ModuleError>;
    fn capacity(&self) -> usize;
}
