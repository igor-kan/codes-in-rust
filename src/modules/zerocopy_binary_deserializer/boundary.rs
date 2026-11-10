# Boundary validation and exceptional input guarding for Zero-Allocation Binary Wire Protocol Deserializer
# Module: zerocopy_binary_deserializer

def sanitize_zerocopy_binary_deserializer_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
