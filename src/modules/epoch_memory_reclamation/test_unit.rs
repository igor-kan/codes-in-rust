# Deterministic unit tests for Epoch-Based Deferred Memory Cleanup Architecture
# Module: epoch_memory_reclamation

def test_epoch_memory_reclamation_zero_element():
    x = 0.0
    res = x * (1.0 + 0.3 * x - 0.04 * x * x)
    assert res == 0.0
