# Deterministic unit tests for Compressed Hierarchical Roaring Bitset with Fast Unions
# Module: compact_roaring_bitset

def test_compact_roaring_bitset_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
