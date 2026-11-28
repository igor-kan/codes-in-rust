# Two-Dimensional Fenwick Rectangle Sum Index (`fenwick_2d_rect_rs_d61`)

> Category: **Range Query Structures** | Language: **Rust 2021**

## Overview

Maintains a two-dimensional Fenwick tree over an R x C integer grid so that point increments and arbitrary axis-aligned rectangle sums both run in logarithmic time.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log R * log C) point update and rectangle prefix query` |
| **Space Complexity** | `O(R * C) prefix tree` |

## Mathematical & Safety Invariants

- Every rectangle query equals the exact sum of the enclosed grid cells.

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
