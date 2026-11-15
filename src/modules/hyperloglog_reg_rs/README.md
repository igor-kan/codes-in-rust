# HyperLogLog Probabilistic Distinct-Count Sketch with LinearCounting (`hyperloglog_reg_rs`)

> Category: **Streaming & Probabilistic Sketches** | Language: **Rust 2021**

## Overview

Estimates cardinality across massive multisets in constant byte registers using Flajolet-Fusy alpha_m harmonic means, leading_zeros() bit counting, and small-range LinearCounting bias correction.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(1) insert, O(2^p) cardinality estimate & union merge` |
| **Space Complexity** | `O(2^p) single-byte leading-zero registers` |

## Mathematical & Safety Invariants

- HyperLogLog register state is a commutative idempotent join-semilattice under elementwise max().
- Duplicates of identical keys never mutate register state or cardinality estimates.

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
