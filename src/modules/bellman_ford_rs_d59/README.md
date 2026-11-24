# Bellman-Ford Shortest Paths & Negative Cycle Option (`bellman_ford_rs_d59`)

> Category: **Graph Shortest Paths** | Language: **Rust 2021**

## Overview

Relaxes negative-weight directed edges over V - 1 sweeps returning None cleanly whenever a negative cycle is reachable.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V * E)` |
| **Space Complexity** | `O(V)` |

## Mathematical & Safety Invariants

- Negative-cycle-free graphs match exact edge-sum relaxation invariants cleanly.

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
