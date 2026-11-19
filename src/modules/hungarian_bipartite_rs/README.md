# Kuhn-Munkres O(N^3) Hungarian Minimum-Cost Assignment in Rust (`hungarian_bipartite_rs`)

> Category: **Combinatorial Optimization** | Language: **Rust 2021**

## Overview

Computes minimum-cost perfect bipartite assignment over N x N cost matrices in O(N^3) time with zero floating-point drift.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N^3) dual-potential augmenting column matching` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Primal matched cost equals dual objective -v[0] identically.

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
