# Ternary Cookbook — Working Demos and Developer Guides for the Ternary Ecosystem

**Ternary Cookbook** is a collection of 11 runnable examples demonstrating real-world applications of ternary {-1, 0, +1} logic. Each example is self-contained and covers a different domain: traffic control, spam filtering, load balancing, cellular automata, climate control, distributed consensus, budget management, signal processing, radiation simulation, proof verification, and full-system composition.

## Why It Matters

Ternary logic is mathematically elegant, but demonstrating its practical value requires concrete, runnable examples. This cookbook bridges theory and practice: every example solves a recognizable problem and can be executed with a single command. The key insight across all examples is that the **0 state is a universal screen** — it prevents pathological lock-in, screens phase transitions, and makes Z₃ cyclic dominance (rock-paper-scissors) the natural coordination mechanism. These aren't toy demos; they implement real algorithms (PID control, RPS game theory, consensus protocols) using ternary primitives.

## How It Works

### Core Types

All examples share a common `Ternary` type (`Minus`, `Zero`, `Plus`) and a `TernaryGrid` for 2D simulations. The grid supports cell counting, neighbor queries, and bulk operations — the substrate for cellular automata and spatial simulations.

### Example Categories

**Control Systems:** `traffic_controller` (ternary traffic lights: go/caution/stop), `thermostat_demo` (PID with ternary heating/cooling/idle), `load_balancer` (health-aware routing with healthy/degraded/down states).

**Classification:** `spam_filter` (ternary classification: spam/ham/unknown), `budget_tracker` (over/under/on-target allocation).

**Distributed Systems:** `consensus_demo` (multi-agent ternary voting), `full_stack` (all systems composed together).

**Simulation:** `game_of_life` (ternary cellular automaton with birth/survival/death), `radiation_sim` (damage propagation with shielded/exposed/safe zones).

**Verification:** `proof_verifier` (ternary proof chains: proved/refuted/unknown), `signal_processor` (DSP pipeline with ternary filters).

### The Ternary Model

Every example uses the mapping: +1 = positive/active/accept, 0 = neutral/idle/abstain, -1 = negative/inactive/reject. This maps naturally to real decisions: Buy/Hold/Sell, Approve/Review/Reject, Heat/Idle/Cool.

## Quick Start

```bash
# Run any example
cargo run --example traffic_controller    # Traffic light simulation
cargo run --example spam_filter           # Ternary spam classification  
cargo run --example game_of_life          # Ternary Game of Life
cargo run --example consensus_demo        # Multi-agent voting
cargo run --example full_stack            # Everything composed

# Use as a library
```

```rust
use ternary_cookbook::{Ternary, TernaryGrid};

let mut grid = TernaryGrid::new(10, 10, 0);
grid.set(5, 5, 1); // Positive cell
let positive_count = grid.count(1);
```

## API

| Type / Function | Description |
|---|---|
| `Ternary` | Enum: `Minus(-1)`, `Zero(0)`, `Plus(1)` |
| `TernaryGrid` | 2D grid with `get(x,y)`, `set(x,y,v)`, `count(state)` |
| 11 examples | Each runnable via `cargo run --example <name>` |

## Architecture Notes

The cookbook serves as the tutorial layer of **SuperInstance**. Each example demonstrates one aspect of the γ + η = C conservation framework: the traffic controller shows ternary state management, the consensus demo shows distributed γ/η balance, and the full_stack example shows the complete conservation law in action. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Wolfram, Stephen. *A New Kind of Science*, Wolfram Media, 2002 — cellular automata.
- Axelrod, Robert. *The Evolution of Cooperation*, Basic Books, 1984 — consensus and cooperation.
- Gardner, Martin. "Mathematical Games: The Fantastic Combinations of John Conway's New Solitaire Game 'Life'," *Scientific American*, 223(4), 1970.

## License

MIT
