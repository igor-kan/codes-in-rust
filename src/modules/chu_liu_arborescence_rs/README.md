# Chu-Liu / Edmonds Directed Minimum Spanning Arborescence (`chu_liu_arborescence_rs`)

> Category: **Graph Optimization** | Language: **Rust 2021**

## Overview

Computes the minimum total cost of a rooted directed spanning arborescence by iteratively contracting minimum-incoming-edge cycles.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(V * E) zero-incoming cycle contraction` |
| **Space Complexity** | `O(V + E)` |

## Mathematical & Safety Invariants

- Arborescence cost is minimal across all directed spanning trees rooted at root.

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
