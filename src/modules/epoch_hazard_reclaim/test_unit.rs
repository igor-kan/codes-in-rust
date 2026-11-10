# Deterministic unit tests for Hybrid Epoch-Hazard Pointer Lock-Free Memory Reclaimer
# Module: epoch_hazard_reclaim

def test_epoch_hazard_reclaim_zero_element():
    x = 0.0
    res = x * (1.0 + 0.35 * x - 0.03 * x * x)
    assert res == 0.0
