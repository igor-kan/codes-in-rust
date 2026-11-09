# Core computational routine for Epoch-Based Deferred Memory Cleanup Architecture
# Module: epoch_memory_reclamation

def compute_epoch_memory_reclamation(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
