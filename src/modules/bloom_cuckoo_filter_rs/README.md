# 4-Way Bucket Cuckoo Filter with Alternate-Bucket Fingerprint Deletion (`bloom_cuckoo_filter_rs`)

> Category: **Probabilistic Membership Structures** | Language: **Rust 2021**

## Overview

Implements Fan et al.'s Cuckoo Filter storing non-zero 16-bit fingerprints across 4-slot buckets with XOR alternate bucket eviction kicks, supporting safe item deletion unlike standard Bloom filters.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(1) lookup & delete, O(1) expected insert` |
| **Space Complexity** | `O(Buckets * 4 * 2 bytes)` |

## Mathematical & Safety Invariants

- contains(k) always returns true for every inserted non-deleted item (zero false negatives).
- Alternate bucket involution holds identically: i ^ alt_hash(fp) -> alt ^ alt_hash(fp) == i.

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
