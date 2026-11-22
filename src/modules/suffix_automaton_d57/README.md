# Online Suffix Automaton (SAM) Distinct Substring Counter (`suffix_automaton_d57`)

> Category: **Automata & Strings** | Language: **Rust 2021**

## Overview

Constructs a deterministic minimal suffix automaton incrementally via suffix links and state cloning, deriving exact distinct substring cardinality in linear time.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N * log Sigma)` |
| **Space Complexity** | `O(N)` |

## Mathematical & Safety Invariants

- Distinct substring counts for all-equal a^N equals N and for distinct length-N equals N*(N+1)/2.

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
