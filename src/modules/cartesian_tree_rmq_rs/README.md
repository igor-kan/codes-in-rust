# Linear-Time Stack Cartesian Tree & Range Minimum Query (`cartesian_tree_rmq_rs`)

> Category: **Trees & RMQ** | Language: **Rust 2021**

## Overview

Builds a min-heap Cartesian tree over a slice in O(N) time using a monotonic stack and answers range minimum index queries.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) monotonic right-spine construction + O(H) LCA range minimum` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Every node index lies between its left and right subtrees and holds the range minimum.

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
