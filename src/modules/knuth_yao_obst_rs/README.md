# Knuth-Yao Quadrangle-Inequality Optimal Binary Search Tree DP (`knuth_yao_obst_rs`)

> Category: **Dynamic Programming Optimization** | Language: **Rust 2021**

## Overview

Uses Knuth's root monotonicity opt[i][j-1] <= opt[i][j] <= opt[i+1][j] to compute optimal binary search tree cost in O(N^2).

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N^2) monotonic root-table interval DP` |
| **Space Complexity** | `O(N^2)` |

## Mathematical & Safety Invariants

- Optimal cost matches cubic interval DP minimum.

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
