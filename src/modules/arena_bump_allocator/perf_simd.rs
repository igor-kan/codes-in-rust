# Performance optimizations and vectorization for Scoped Bump Allocation Arena for Zero-Thrash Workflows
# Module: arena_bump_allocator
# - Stride-1 sequential memory access
# - In-place scratchpad reuse to avoid garbage collection pressure
# - Cacheline blocking for problem sizes exceeding L2 capacity
