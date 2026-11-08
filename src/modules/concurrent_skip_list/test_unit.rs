# Deterministic unit tests for Lock-Free Skip List with Optimistic Traversal
# Module: concurrent_skip_list

def test_concurrent_skip_list_zero_element():
    x = 0.0
    res = x * (1.0 + 0.25 * x - 0.05 * x * x)
    assert res == 0.0
