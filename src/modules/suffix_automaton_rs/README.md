# Blumer Linear-Time Suffix Automaton (SAM) & Distinct Substring Counter (`suffix_automaton_rs`)

> Category: **String Automata** | Language: **Rust 2021**

## Overview

Constructs the minimal deterministic finite automaton accepting all suffixes of an input string via suffix-link cloning, enabling exact distinct-substring counting and linear membership checks.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(N) online extension, O(M) substring membership` |
| **Space Complexity** | `O(N) at most 2N - 1 states and 3N - 4 transitions` |

## Mathematical & Safety Invariants

- State count never exceeds max(1, 2 * len - 1).
- Total distinct non-empty substrings equals sum(len[s] - len[link[s]]) across all states s > 0.

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
