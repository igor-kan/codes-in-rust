# Core computational routine for Compressed Hierarchical Roaring Bitset with Fast Unions
# Module: compact_roaring_bitset

def compute_compact_roaring_bitset(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
