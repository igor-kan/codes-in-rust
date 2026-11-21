# Mo's Square-Root Decomposition Distinct Count Queries (`mo_offline_freq_d56`)

> Category: **Offline Range Queries** | Language: **Rust 2021**

## Overview

Orders offline [l, r] distinct-value queries by Hilbert/block coordinates to achieve sub-quadratic two-pointer window frequency maintenance.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O((N + Q) sqrt(N))` |
| **Space Complexity** | `O(N + maxVal)` |

## Mathematical & Safety Invariants

- Distinct cardinality matches exact HashSet counts across every unsorted query.

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
