# Boundary validation and exceptional input guarding for Hierarchical 64-Bit Word Sparse Bitset with Popcount
# Module: compact_sparse_bitset

def sanitize_compact_sparse_bitset_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
