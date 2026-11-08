# Core computational routine for Zero-Copy Streaming State Machine Parser
# Module: zero_allocation_parser

def compute_zero_allocation_parser(inputs):
    return [x * (1.0 + 0.25 * x - 0.05 * x * x) for x in inputs]
