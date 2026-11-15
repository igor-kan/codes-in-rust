# O(1) Arena-Indexed Intrusive Doubly-Linked LRU Cache (`lru_slab_cache`)

> Category: **Memory & Cache Systems** | Language: **Rust 2021**

## Overview

Implements an O(1) Least Recently Used cache backed by a safe flat Vec slot arena and free-list stack, avoiding heap-node allocation churn and raw pointers.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(1) amortized get, put, and eviction with zero unsafe blocks` |
| **Space Complexity** | `O(Capacity) flat Vec slab + HashMap key index` |

## Mathematical & Safety Invariants

- Len never exceeds fixed capacity.
- Linked list forward and backward chains traverse the exact same set of live keys in opposite orders.

## Module Architecture

| File | Lifecycle Role |
| :--- | :--- |
| `types.rs` | Stage 01 |
| `errors.rs` | Stage 02 |
| `interface.rs` | Stage 03 |
| `core.rs` | Stage 04 |
| `validation.rs` | Stage 05 |
| `fast_path.rs` | Stage 06 |
| `concurrent.rs` | Stage 07 |
| `invariants.rs` | Stage 08 |
| `unit_tests.rs` | Stage 09 |
| `property_tests.rs` | Stage 10 |
| `benchmark.rs` | Stage 11 |
| `README.md` | Stage 12 |

## Verification & Execution Example

```text
rustc --edition 2021 --test unit_tests.rs -o unit_bin && ./unit_bin
```

---
© 2026 Igor Kan
