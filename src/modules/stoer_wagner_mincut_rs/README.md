# Stoer-Wagner Global Undirected Min-Cut Solver in Rust (`stoer_wagner_mincut_rs`)

> Category: **Graph Cuts** | Language: **Rust 2021**

## Overview

Computes the global minimum weight cut over undirected graphs via Stoer-Wagner contraction phases.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V^3) maximum-adjacency phase contraction` |
| **Space Complexity** | `O(V^2)` |

## Mathematical & Safety Invariants

- Global min-cut weight <= minimum weighted vertex degree.

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
