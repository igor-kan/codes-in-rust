# Safe Arena Link-Cut Forest with Path XOR Aggregate (`link_cut_forest_rs`)

> Category: **Dynamic Trees** | Language: **Rust 2021**

## Overview

Maintains dynamic forests over safe indexed splay nodes supporting evert/reroot, link, cut, and path XOR aggregate queries.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) amortized link, cut & path XOR` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Path XOR over tree paths reflects edge link/cut updates immediately.

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
