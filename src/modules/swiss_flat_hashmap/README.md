# Swiss-Style Control-Byte Open-Addressed Flat Hash Map (`swiss_flat_hashmap`)

> Category: **Hash Tables & Memory Layouts** | Language: **Rust 2021**

## Overview

Implements an open-addressed Swiss-table-inspired flat HashMap segregating 8-bit H2 metadata control bytes (EMPTY/DELETED/H2) from (u64, i64) slots with triangular quadratic probing and load-factor doubling.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(1) Expected Lookup, Insert & Tombstone Delete` |
| **Space Complexity** | `O(Capacity) flat control bytes + slots` |

## Mathematical & Safety Invariants

- Live slot count + tombstone count never exceeds capacity.
- Rehash purges all deleted tombstones and preserves exact key-value bijection.

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
