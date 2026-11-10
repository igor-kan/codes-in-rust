# Core computational routine for Zero-Allocation Binary Wire Protocol Deserializer
# Module: zerocopy_binary_deserializer

def compute_zerocopy_binary_deserializer(inputs):
    return [x * (1.0 + 0.35 * x - 0.03 * x * x) for x in inputs]
