# O(1) Static Idempotent Range Minimum Table (`sparse_table_idemp_d56`)

> Category: **Range Queries** | Language: **Rust 2021**

## Overview

Precomputes overlapping power-of-two block minima across static slices so arbitrary closed ranges [l, r] answer in constant O(1) two-lookup time.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log N) build, O(1) query` |
| **Space Complexity** | `O(N log N)` |

## Mathematical & Safety Invariants

- Range min agrees identically with linear scan over every valid [l, r] slice.

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
