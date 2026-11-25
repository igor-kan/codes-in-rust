# Monotonic Integer-Keyed Dijkstra Solver with Indexed Min-Heap (`dijkstra_radix_heap_d60`)

> Category: **Shortest Paths** | Language: **Rust 2021**

## Overview

Solves non-negative integer weighted single-source shortest paths while verifying monotonic bucket distances across extracted vertices.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O((V + E) log V) monotonic edge relaxation` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Settled vertex distances are monotonically non-decreasing.

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
