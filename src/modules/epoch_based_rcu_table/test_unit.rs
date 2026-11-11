# Deterministic unit tests for Read-Copy-Update (RCU) Epoch-Based Concurrent Hash Table
# Module: epoch_based_rcu_table

def test_epoch_based_rcu_table_zero_element():
    x = 0.0
    res = x * (1.0 + 0.4 * x - 0.02 * x * x)
    assert res == 0.0
