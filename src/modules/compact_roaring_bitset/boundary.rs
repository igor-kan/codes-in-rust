# Boundary validation and exceptional input guarding for Compressed Hierarchical Roaring Bitset with Fast Unions
# Module: compact_roaring_bitset

def sanitize_compact_roaring_bitset_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
