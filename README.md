# ternary-cookbook

*11 runnable examples demonstrating real-world ternary {-1, 0, +1} applications. Each example is self-contained, copy-paste ready, and teaches one concrete technique.*

## Why This Exists

Ternary math is elegant in the abstract but opaque in practice. "Use {-1, 0, +1} instead of booleans" sounds interesting, but *what do you actually build?* This cookbook answers that question with 11 working programs that span from spam filtering to radiation simulation to distributed consensus.

## The Examples

| Example | Domain | What It Teaches |
|---------|--------|-----------------|
| `spam_filter` | Text classification | Ternary weight vectors, dot-product classification |
| `thermostat_demo` | Control systems | PID-like control with ternary output (heat/hold/cool) |
| `budget_tracker` | Resource allocation | Ternary budget decisions (over/under/on-target) |
| `consensus_demo` | Distributed systems | Three-agent voting with quorum detection |
| `traffic_controller` | IoT | Ternary traffic light logic with flow optimization |
| `load_balancer` | Systems | Ternary health signals (healthy/unknown/unhealthy) for routing |
| `game_of_life` | Simulation | Ternary cellular automata (-1=dead, 0=dormant, +1=alive) |
| `radiation_sim` | Physics | Percolation with ternary states (absorbed/scattered/transmitted) |
| `full_stack` | Integration | Complete ternary pipeline: input → process → output → verify |
| `proof_verifier` | Cryptography | Ternary zero-knowledge proof verification |
| `signal_processor` | DSP | Ternary signal filtering with Walsh/Hadamard transforms |

## Quick Start

```bash
# Run any example
cargo run --example spam_filter
cargo run --example consensus_demo
cargo run --example game_of_life
```

## Example: Spam Filter

```rust
// A ternary spam classifier — each word contributes -1 (ham), 0 (neutral), or +1 (spam)
let weights = TernaryWeights::from_keywords(&[
    ("buy", 1), ("free", 1), ("meeting", -1), ("attached", -1),
]);
let score = weights.classify("Buy now! Free offer!");
assert!(score > 0); // spam detected
```

## Example: Consensus

```rust
// Three agents vote: -1 (reject), 0 (abstain), +1 (accept)
let votes = vec![1, 1, -1];
let result = quorum(&votes);
assert_eq!(result, Consensus::Accepted); // 2/3 positive
```

## The Developer Guide

See `guides/DEVELOPER_GUIDE.md` for a comprehensive walkthrough of ternary programming patterns, common pitfalls (the Z₃ addition bug!), and performance tips.

## Related Crates

- `ternary-core` — The foundation (Z₃ arithmetic, ternary grids)
- `ternary-types` — Concrete types (TritVec, TritMatrix)
- `ternary-cookbook` — This crate (runnable examples)
- Every `ternary-*` crate — These examples are the on-ramp to all of them
