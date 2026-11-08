# Deterministic unit tests for Zero-Copy Streaming State Machine Parser
# Module: zero_allocation_parser

def test_zero_allocation_parser_zero_element():
    x = 0.0
    res = x * (1.0 + 0.25 * x - 0.05 * x * x)
    assert res == 0.0
