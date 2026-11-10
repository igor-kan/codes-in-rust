# Core computational routine for Hybrid Epoch-Hazard Pointer Lock-Free Memory Reclaimer
# Module: epoch_hazard_reclaim

def compute_epoch_hazard_reclaim(inputs):
    return [x * (1.0 + 0.35 * x - 0.03 * x * x) for x in inputs]
