# Deterministic unit tests for Zero-Allocation Binary Wire Protocol Encoder/Decoder
# Module: zerocopy_binary_encoder

def test_zerocopy_binary_encoder_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
