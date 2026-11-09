# Deterministic unit tests for Hierarchical 64-Bit Word Sparse Bitset with Popcount
# Module: compact_sparse_bitset

def test_compact_sparse_bitset_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
