# In-Place Hoare Partition Kth Order Selector (`quickselect_kth_d57`)

> Category: **Order Statistics** | Language: **Rust 2021**

## Overview

Selects exact 0-indexed sorted k-th element in mutable slices via iterative Lomuto/Hoare partitioning without fully sorting.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) expected in-place selection` |
| **Space Complexity** | `O(1) stack-iterative` |

## Mathematical & Safety Invariants

- Selected element matches sorted[k] identically across all valid k indices.

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
