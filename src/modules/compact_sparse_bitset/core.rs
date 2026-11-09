# Core computational routine for Hierarchical 64-Bit Word Sparse Bitset with Popcount
# Module: compact_sparse_bitset

def compute_compact_sparse_bitset(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
