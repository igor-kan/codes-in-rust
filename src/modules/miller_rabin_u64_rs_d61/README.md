# Deterministic Miller-Rabin 64-Bit Primality Tester (`miller_rabin_u64_rs_d61`)

> Category: **Number Theory** | Language: **Rust 2021**

## Overview

Tests 64-bit primality with deterministic Miller-Rabin witnesses and u128 modular multiplication, avoiding any probabilistic false positives.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(log^3 N) modular exponentiation with 12 fixed bases` |
| **Space Complexity** | `O(1)` |

## Mathematical & Safety Invariants

- All reported primes pass trial division and the strong probable-prime witness loop.

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
