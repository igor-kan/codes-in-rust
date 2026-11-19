# Johnson Potential-Reweighted Sparse All-Pairs Shortest Paths (`johnsons_apsp_binary_rs`)

> Category: **Shortest Paths** | Language: **Rust 2021**

## Overview

Handles negative-weight acyclic graphs via Bellman-Ford potential shifting and non-negative BinaryHeap Dijkstra passes from each source.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V*E + V*E log V) Bellman-Ford potentials + BinaryHeap Dijkstra` |
| **Space Complexity** | `O(V^2)` |

## Mathematical & Safety Invariants

- Reweighted edge weights stay non-negative and restore exact shortest distances.

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
