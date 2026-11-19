# Bottom-Up Cache-Local ZKW Iterative Segment Tree in Rust (`zkw_bottom_up_seg_rs`)

> Category: **Range Data Structures** | Language: **Rust 2021**

## Overview

Indexes a flattened power-of-two segment tree from base offset M upward for fast point updates and range sum queries.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) recursion-free iterative range query & update` |
| **Space Complexity** | `O(2 * N)` |

## Mathematical & Safety Invariants

- Iterative segment query equals exact subarray sum.

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
