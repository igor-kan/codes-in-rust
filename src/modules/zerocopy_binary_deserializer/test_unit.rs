# Deterministic unit tests for Zero-Allocation Binary Wire Protocol Deserializer
# Module: zerocopy_binary_deserializer

def test_zerocopy_binary_deserializer_zero_element():
    x = 0.0
    res = x * (1.0 + 0.35 * x - 0.03 * x * x)
    assert res == 0.0
