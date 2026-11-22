# Iterative Bottom-Up 2N Segment Tree Point Update & Min Query (`segment_min_point_d57`)

> Category: **Segment Trees** | Language: **Rust 2021**

## Overview

Stores an iterative binary tree over 2N backing vector slots supporting cache-friendly point assignments and half-open [l, r) range minima.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) update and range min query` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Half-open [l, r) minimum agrees identically with linear scan after updates.

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
