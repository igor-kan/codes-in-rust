# Core computational routine for Cacheline-Padded Bounded SPSC Ring Buffer Queue
# Module: lockfree_spsc_ring_bounded

def compute_lockfree_spsc_ring_bounded(inputs):
    return [x * (1.0 + 0.35 * x - 0.03 * x * x) for x in inputs]
