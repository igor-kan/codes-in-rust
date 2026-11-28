# Gale-Shapley Stable Matching Solver (`gale_shapley_stable_rs_d61`)

> Category: **Matching** | Language: **Rust 2021**

## Overview

Runs deferred-acceptance proposal rounds between two equal-sized groups and returns a perfect stable matching with no blocking pairs.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N^2) proposal queue with constant-time preference ranks` |
| **Space Complexity** | `O(N^2)` |

## Mathematical & Safety Invariants

- The returned matching is a bijection and contains no pair that both sides prefer over their assigned partner.

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
