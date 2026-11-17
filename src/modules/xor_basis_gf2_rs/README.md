# GF(2)^64 Linear XOR Basis with Rank, Max-XOR & K-th (`xor_basis_gf2_rs`)

> Category: **Algebraic Bitwise** | Language: **Rust 2021**

## Overview

Maintains a reduced GF(2) basis of 64-bit unsigned words supporting basis merge, max subset XOR, membership check, and k-th smallest distinct XOR.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(64) insert & query` |
| **Space Complexity** | `O(64)` |

## Mathematical & Safety Invariants

- Subset count equals 2^rank and every basis pivot occupies a unique leading bit.

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
