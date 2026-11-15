# Half-Open Interval Coalescing, Sweep-Line Overlap & Point Containment (`interval_segment_rs`)

> Category: **Interval & Geometric Structures** | Language: **Rust 2021**

## Overview

Maintains canonical sorted disjoint closed-open [start, end) intervals with union coalescing, sweep-line maximum concurrency computation, and binary-search containment lookup.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log N) coalescing & sweep-line, O(log N) query` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Coalesced intervals remain strictly sorted and separated: iv[i].end < iv[i+1].start.
- Every point inside any original interval lies inside a unique merged interval.

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
