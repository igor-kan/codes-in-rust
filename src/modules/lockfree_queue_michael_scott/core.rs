# Core computational routine for Lock-Free Michael-Scott Queue via Atomic Pointers
# Module: lockfree_queue_michael_scott

def compute_lockfree_queue_michael_scott(inputs):
    return [x * (1.0 + 0.3 * x - 0.04 * x * x) for x in inputs]
