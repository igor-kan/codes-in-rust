# O(1) Semigroup Disjoint Sparse Table (Non-Idempotent Range Queries) (`disjoint_sparse_table`)

> Category: **Range Query Structures** | Language: **Rust 2021**

## Overview

Builds hierarchical midpoint suffix/prefix tables across power-of-two padded blocks, answering arbitrary associative semigroup queries (modulo products, affine transforms) in O(1) without overlapping elements.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log N) build, O(1) arbitrary associative semigroup query` |
| **Space Complexity** | `O(N log N)` |

## Mathematical & Safety Invariants

- Query(l, r) uses at most one semigroup combine without double-counting any element.
- Matches O(N) associative fold across all 0 <= l <= r < N subranges.

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
