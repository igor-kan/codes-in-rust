# Zero-Allocation KMP Non-Overlapping Match Counter (`knuth_morris_pratt_d57`)

> Category: **Pattern Counting** | Language: **Rust 2021**

## Overview

Computes exact KMP border shifts over arbitrary byte slices and reports maximum non-overlapping pattern occurrence counts cleanly.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(|T| + |P|)` |
| **Space Complexity** | `O(|P|)` |

## Mathematical & Safety Invariants

- Non-overlapping occurrence count never exceeds exact overlap match count.

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
