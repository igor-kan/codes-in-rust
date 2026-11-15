# Google Maglev Consistent Hashing Permutation Lookup Table (`consistent_maglev`)

> Category: **Distributed Systems** | Language: **Rust 2021**

## Overview

Implements Google's Maglev round-robin preference permutation table over a prime lookup modulus M, achieving near-uniform backend load balance and minimal key remapping on backend removal.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(M * N) table build (prime M), O(1) lookup, minimal disruption` |
| **Space Complexity** | `O(M) flat backend ID table` |

## Mathematical & Safety Invariants

- Every slot in the M-entry lookup table maps to a valid backend index in 0..N.
- Removing a backend remaps primarily keys owned by that backend while keeping > (N-1)/N of other keys fixed.

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
