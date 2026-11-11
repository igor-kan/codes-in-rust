# Core computational routine for Zero-Allocation Binary Wire Protocol Encoder/Decoder
# Module: zerocopy_binary_encoder

def compute_zerocopy_binary_encoder(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
