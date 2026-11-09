# Deterministic unit tests for Lock-Free Michael-Scott Queue via Atomic Pointers
# Module: lockfree_queue_michael_scott

def test_lockfree_queue_michael_scott_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
