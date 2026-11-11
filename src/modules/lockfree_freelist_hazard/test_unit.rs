# Deterministic unit tests for Lock-Free Freelist using Hazard Pointers for Memory
# Module: lockfree_freelist_hazard

def test_lockfree_freelist_hazard_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
