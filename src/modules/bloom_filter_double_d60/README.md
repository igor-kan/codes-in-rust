# Kirsch-Mitzenmacher Double-Hashing Compact Bloom Filter (`bloom_filter_double_d60`)

> Category: **Probabilistic Membership** | Language: **Rust 2021**

## Overview

Uses Kirsch-Mitzenmacher linear combinations h1 + i*h2 over two 64-bit SplitMix hashes to pack a K-probe Bloom filter inside dense u64 words.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(k) bit probes per insert/contains with zero heap hash allocations` |
| **Space Complexity** | `O(M) bit-packed u64 words` |

## Mathematical & Safety Invariants

- Guarantees zero false negatives across all inserted 64-bit elements.

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
