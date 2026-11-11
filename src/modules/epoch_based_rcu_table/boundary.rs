# Boundary validation and exceptional input guarding for Read-Copy-Update (RCU) Epoch-Based Concurrent Hash Table
# Module: epoch_based_rcu_table

def sanitize_epoch_based_rcu_table_input(x, min_v=-1e6, max_v=1e6):
    if x != x:
        return 0.0
    return max(min_v, min(max_v, x))
