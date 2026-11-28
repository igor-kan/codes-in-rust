# Douglas-Peucker Polyline Simplification Engine (`douglas_peucker_rs_d61`)

> Category: **Computational Geometry** | Language: **Rust 2021**

## Overview

Simplifies a polyline by recursively retaining the farthest point from each chord whenever its perpendicular deviation exceeds epsilon.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log N) average recursive segment splitting` |
| **Space Complexity** | `O(N) keep flags and segment stack` |

## Mathematical & Safety Invariants

- The first and last vertices are always preserved and every removed vertex lies within epsilon of a retained segment.

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
