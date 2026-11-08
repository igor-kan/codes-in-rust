# Performance optimizations and vectorization for Criterion Statistical Microbenchmarking Harness
# Module: criterion_microbenchmarks
# - Stride-1 sequential memory access
# - In-place scratchpad reuse to avoid garbage collection pressure
# - Cacheline blocking for problem sizes exceeding L2 capacity
