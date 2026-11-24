# Lazy-Propagation Range-Add & Range-Sum Segment Tree (`lazy_seg_add_rs_d59`)

> Category: **Range Queries** | Language: **Rust 2021**

## Overview

Maintains mutable range-add interval accumulations and exact inclusive [l, r] sums in 4N segment arrays with deferred lazy pushes.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) range add and query` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Closed range sum after batch interval adds agrees cleanly with elementwise vector sums.

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
