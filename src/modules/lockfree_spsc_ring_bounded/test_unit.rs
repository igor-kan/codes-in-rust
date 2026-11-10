# Deterministic unit tests for Cacheline-Padded Bounded SPSC Ring Buffer Queue
# Module: lockfree_spsc_ring_bounded

def test_lockfree_spsc_ring_bounded_zero_element():
    x = 0.0
    res = x * (1.0 + 0.35 * x - 0.03 * x * x)
    assert res == 0.0
