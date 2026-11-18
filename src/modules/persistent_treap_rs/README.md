# Arena-Backed Path-Copied Persistent Implicit Treap in Rust (`persistent_treap_rs`)

> Category: **Persistent Structures** | Language: **Rust 2021**

## Overview

Maintains immutable sequence versions via arena-indexed functional treap split and merge without Rc overhead.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) functional split, merge, and range sum` |
| **Space Complexity** | `O(N + Q log N)` |

## Mathematical & Safety Invariants

- Earlier version roots preserve their exact elements and sums across updates.

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
