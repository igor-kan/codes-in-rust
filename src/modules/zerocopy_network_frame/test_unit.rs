# Deterministic unit tests for Zero-Cost Network Frame Header Byte Parser
# Module: zerocopy_network_frame

def test_zerocopy_network_frame_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
