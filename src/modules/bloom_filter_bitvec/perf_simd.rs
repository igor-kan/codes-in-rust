# Performance optimizations and vectorization for Cache-Aligned Bitvector Bloom Filter with SipHash
# Module: bloom_filter_bitvec
# - Stride-1 sequential memory access
# - In-place scratchpad reuse to avoid garbage collection pressure
# - Cacheline blocking for problem sizes exceeding L2 capacity
