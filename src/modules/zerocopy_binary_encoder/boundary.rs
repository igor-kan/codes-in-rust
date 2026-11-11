# Boundary validation and exceptional input guarding for Zero-Allocation Binary Wire Protocol Encoder/Decoder
# Module: zerocopy_binary_encoder

def sanitize_zerocopy_binary_encoder_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
