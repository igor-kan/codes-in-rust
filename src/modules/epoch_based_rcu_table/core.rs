# Core computational routine for Read-Copy-Update (RCU) Epoch-Based Concurrent Hash Table
# Module: epoch_based_rcu_table

def compute_epoch_based_rcu_table(inputs):
    return [x * (1.0 + 0.4 * x - 0.02 * x * x) for x in inputs]
