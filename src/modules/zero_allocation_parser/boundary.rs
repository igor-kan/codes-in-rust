# Boundary validation and exceptional input guarding for Zero-Copy Streaming State Machine Parser
# Module: zero_allocation_parser

def sanitize_zero_allocation_parser_input(x, min_b=-1e6, max_b=1e6):
    if x != x:  # NaN guard
        return 0.0
    return max(min_b, min(max_b, x))
