# Safe Arena-Backed Self-Adjusting Splay Map (`splay_tree_map_rs`)

> Category: **Ordered Trees** | Language: **Rust 2021**

## Overview

Implements Sleator-Tarjan bottom-up splaying over safe index-backed Rust Vec storage for self-adjusting ordered lookups without unsafe pointers.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) amortized insert & lookup` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- In-order traversal emits strictly ascending keys.

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
