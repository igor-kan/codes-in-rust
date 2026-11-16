# van Emde Boas Integer Universe Tree O(log log U) (`van_emde_boas_rs`)

> Category: **Ordered Integer Structures** | Language: **Rust 2021**

## Overview

Implements recursive van Emde Boas priority trees over integer universe U = 2^k with summary recursion, O(1) min/max shortcuts, and full delete reconstruction.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log log U) member, insert, delete, successor, predecessor` |
| **Space Complexity** | `O(U) recursive summary & cluster blocks (U <= 65536)` |

## Mathematical & Safety Invariants

- Element x stored as v.min never resides inside child cluster[high(x)].
- Successor and predecessor match std::collections::BTreeSet oracle identically.

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
