# Boruvka Minimum Spanning Forest Builder (`boruvka_mst_rs_d61`)

> Category: **Graph Algorithms** | Language: **Rust 2021**

## Overview

Builds a minimum spanning forest by repeatedly selecting the cheapest outgoing edge of every component and merging components through a union-find structure.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(E log V) component-minimum edge merging` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Every selected edge joins two distinct components and the total weight matches Kruskal on the same graph.

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
