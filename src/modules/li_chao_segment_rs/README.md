# Dynamic Li Chao Segment Tree for Lower-Envelope Line Queries (`li_chao_segment_rs`)

> Category: **Convex Hull Optimization** | Language: **Rust 2021**

## Overview

Stores dominant affine lines y = m*x + b at segment midpoints over [0, X) to answer lower-envelope queries in O(log X).

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log X) line insertion and point minimum evaluation` |
| **Space Complexity** | `O(X)` |

## Mathematical & Safety Invariants

- Point query returns exact min_i (m_i * x + b_i) across all inserted lines.

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
