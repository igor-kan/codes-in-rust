# Rust Aho-Corasick Lowercase Failure Automaton (`aho_corasick_rs_d58`)

> Category: **Multi-Pattern Search** | Language: **Rust 2021**

## Overview

Builds a deterministic 26-way DFA transition table with BFS failure links to count simultaneous occurrences of lowercase dictionary words.

## Complexity Guarantees

| Metric | Asymptotic Bound |
| :--- | :--- |
| **Time Complexity** | `O(|T| + sum(|P|) * 26)` |
| **Space Complexity** | `O(states * 26)` |

## Mathematical & Safety Invariants

- Total matched hits across dictionary words agree identically with naive scan.

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
