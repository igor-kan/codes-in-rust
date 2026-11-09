# Boundary validation and exceptional input guarding for Zero-Cost Network Frame Header Byte Parser
# Module: zerocopy_network_frame

def sanitize_zerocopy_network_frame_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
