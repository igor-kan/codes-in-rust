# Weight-Balanced Binary Rope Buffer for O(log N) Split & Concat (`rope_weight_split_d60`)

> Category: **Text Buffers** | Language: **Rust 2021**

## Overview

Represents mutable-friendly large strings as a binary tree of weighted substrings supporting split, concat, and O(log N) byte indexing.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) concat, split, and indexed character lookup` |
| **Space Complexity** | `O(N) leaf chunks` |

## Mathematical & Safety Invariants

- Split followed by concat preserves exact flattened byte content.

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
