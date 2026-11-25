# Ordered Chunked Block Index with Binary-Search Range Scan (`bplus_leaf_index_d60`)

> Category: **Storage Engines** | Language: **Rust 2021**

## Overview

Maintains sorted fixed-size leaf blocks splitting automatically at capacity B and answering closed key-range queries cleanly.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log N) search + cache-friendly leaf range scan` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Keys stay strictly sorted across all leaf splits and range scans.

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
