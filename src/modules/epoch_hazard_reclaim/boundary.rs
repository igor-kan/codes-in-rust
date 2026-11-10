# Boundary validation and exceptional input guarding for Hybrid Epoch-Hazard Pointer Lock-Free Memory Reclaimer
# Module: epoch_hazard_reclaim

def sanitize_epoch_hazard_reclaim_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
