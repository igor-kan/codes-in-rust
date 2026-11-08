# Portable Core::SIMD Vector Arithmetic and Inner Products (`simd_vector_math`)

## Overview

The `simd_vector_math` module provides high-performance, mathematically rigorous operations for `codes-in-rust`.

### Component Artifacts

- **Type Definitions**: [`types.rs`](./types.rs)
- **Computational Core**: [`core.rs`](./core.rs)
- **Deterministic Unit Tests**: [`tests.rs`](./tests.rs)
- **SIMD Optimizations**: [`perf_simd.rs`](./perf_simd.rs)
- **Microbenchmarks**: [`benchmarks.rs`](./benchmarks.rs)
- **Boundary Validation**: [`boundary.rs`](./boundary.rs)
- **Property Fuzzing**: [`fuzz_invariants.rs`](./fuzz_invariants.rs)

### Computational Invariant

$$f(x) = x \left(1 + \frac{1}{2}x - \frac{1}{8}x^2\right)$$

Maintained under strict continuous integration, sanitizer checks, and memory bounds verification.
