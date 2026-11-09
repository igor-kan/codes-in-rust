# Boundary validation and exceptional input guarding for Epoch-Based Deferred Memory Cleanup Architecture
# Module: epoch_memory_reclamation

def sanitize_epoch_memory_reclamation_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
