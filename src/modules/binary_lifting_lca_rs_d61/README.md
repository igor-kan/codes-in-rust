# Binary-Lifting Lowest Common Ancestor Table (`binary_lifting_lca_rs_d61`)

> Category: **Tree Queries** | Language: **Rust 2021**

## Overview

Precomputes power-of-two ancestor jumps for every tree vertex and answers lowest-common-ancestor queries by levelling depths and descending the jump table.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N log N) table build + O(log N) LCA query` |
| **Space Complexity** | `O(N log N)` |

## Mathematical & Safety Invariants

- For every pair of vertices the returned ancestor lies on both root paths and has maximal depth.

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
