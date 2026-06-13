# Ternary Cookbook

Working demos, tutorials, and developer guides for the **ternary {-1, 0, +1} ecosystem**. Eleven runnable examples that demonstrate real-world applications — from traffic control to spam filtering to radiation damage simulation — all built on ternary logic primitives.

## Why It Matters

Theoretical frameworks need concrete demonstrations. The ternary ecosystem spans signal processing, agent systems, economics, and biology — but the unifying insight is that **three-valued logic** (negative/neutral/positive) maps naturally to real decisions:

- Buy / Hold / Sell
- Approve / Review / Reject  
- Heat / Idle / Cool
- Healthy / Degraded / Down

The key discovery from this collection: **the 0 state is a universal screen**. It prevents pathological lock-in, screens phase transitions, and makes Z₃ cyclic dominance (rock-paper-scissors) the natural coordination mechanism. Every example demonstrates this principle in a different domain.

## How It Works

### Core Abstractions

The cookbook provides shared types used across all examples:

**Ternary enum**: The fundamental {-1, 0, +1} value with display labels `[-]`, `[ ]`, `[+]`.

**TernaryGrid**: A 2D grid of ternary values with rendering (`░` for -1, `·` for 0, `█` for +1) and neighbor queries for cellular automata.

**TernaryPid**: A PID controller that ternarizes its output — the continuous PID formula is computed, then reduced to a ternary signal:

$$u(t) = K_p e(t) + K_i \int_0^t e(\tau) d\tau + K_d \frac{de}{dt}$$
$$\text{output} = \text{sign}(u) \cdot \mathbb{1}[|u| > \text{deadband}]$$

The deadband near zero prevents oscillation — the same principle as the ternary accumulator's momentum threshold.

### Mathematical Foundations

**Moving average** of ternary signals:

$$\bar{x}_t = \frac{1}{W} \sum_{i=t-W+1}^{t} x_i, \quad x_i \in \{-1, 0, +1\}$$

**Zero crossing rate** — measures signal volatility:

$$Z = \frac{1}{N-1} \sum_{t=1}^{N-1} \mathbb{1}[\text{sign}(x_t) \neq \text{sign}(x_{t-1})]$$

**Ternary Game of Life** — three-age-structure cellular automaton:
- Young (+1): survives with 2-3 positive neighbors, else dies to 0
- Dead (0): born with exactly 3 positive neighbors, infected with 3+ negative
- Old (-1): persists with 2+ negative neighbors, else dies

This extends Conway's Game of Life with a death/aging phase, creating Z₃ cyclic dynamics.

### Complexity

| Operation | Time |
|-----------|------|
| `TernaryGrid::render()` | O(W·H) |
| `step_game_of_life(grid)` | O(W·H) |
| `TernaryPid::update()` | O(1) |
| `moving_average(N, W)` | O(N) |
| `zero_crossing_rate(N)` | O(N) |

## Quick Start

```bash
# Run any of the 11 examples:
cargo run --example traffic_controller    # State machine traffic light
cargo run --example spam_filter           # Ternary classification
cargo run --example load_balancer         # Health-aware routing
cargo run --example game_of_life          # Ternary cellular automaton
cargo run --example thermostat_demo       # PID climate control
cargo run --example consensus_demo        # Multi-agent voting
cargo run --example budget_tracker        # Resource allocation
cargo run --example signal_processor      # DSP pipeline
cargo run --example radiation_sim         # Damage propagation
cargo run --example proof_verifier        # Proof chain verification
cargo run --example full_stack            # All systems composed
```

```rust
use ternary_cookbook::{Ternary, TernaryGrid, TernaryPid, step_game_of_life};

// Ternary PID controller
let mut pid = TernaryPid::new(2.0, 0.5, 1.0).with_deadband(0.5);
let action = pid.update(72.0, 68.0); // setpoint, measurement
// action ∈ {-1, 0, +1}: cool, idle, heat

// Ternary Game of Life
let mut grid = TernaryGrid::new(10, 10, 0);
grid.set(5, 5, 1); grid.set(5, 6, 1); grid.set(5, 7, 1);
let next_gen = step_game_of_life(&grid);
println!("{}", next_gen.render());
```

## API

### Core Types

| Type | Description |
|------|-------------|
| `Ternary` | Enum: Minus (-1), Zero (0), Plus (+1) |
| `TernaryGrid` | 2D grid with rendering and neighbor queries |
| `TernaryPid` | PID controller with ternary output |
| `SimpleRng` | Deterministic LCG random number generator |

### Functions

| Function | Description |
|--------|-------------|
| `step_game_of_life(grid) → TernaryGrid` | One generation of ternary Life |
| `moving_average(samples, window) → Vec<f64>` | Sliding window average |
| `zero_crossing_rate(samples) → f64` | Sign-change frequency |

## Architecture Notes

The cookbook demonstrates the **γ + η = C** conservation link across domains:

- **γ (structure)**: the fixed decision architecture — three states, fixed thresholds, grid topology
- **η (dynamics)**: the perturbation stream — sensor readings, user inputs, neighbor interactions
- **C (conservation)**: domain-specific invariants — traffic throughput, temperature bounds, population counts

Each example shows how constraining decisions to ternary space produces more robust, interpretable systems. The 0 state (Explore, Hold, Review) is never a no-op — it is the active equilibrium that prevents runaway dynamics.

## References

- Conway, J. (1970). *The Game of Life*. — Binary cellular automaton, here extended to ternary.
| Åström, K.J. & Murray, R.M. (2008). *Feedback Systems*. Princeton — PID control theory.
| Shannon, C.E. (1948). *A Mathematical Theory of Communication*. — Information entropy.
| von Neumann, J. (1966). *Theory of Self-Reproducing Automata*. — Cellular automata foundations.

## License: MIT
