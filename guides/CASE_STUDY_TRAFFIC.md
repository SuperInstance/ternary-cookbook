# Ternary in Practice: Building the Traffic Controller

*A case study in ternary state machines — from binary frustration to ternary clarity.*

---

## The Problem

You need a traffic light controller. Three states: red, yellow, green. They cycle in a fixed order, with different timing for each transition.

A binary state machine needs at least 2 bits to represent 3 states. That means **4 possible states**, one of which is invalid:

```rust
// 2 bits, 4 states, 1 undefined
enum BinaryLight {
    Red,     // 00
    Yellow,  // 01
    Green,   // 10
    // 11 — What happens here?
}
```

What happens when a cosmic ray flips both bits and you land on `11`? Either undefined behavior, or you add error handling for something that shouldn't happen.

## The Ternary Approach

Three states, one encoding. No dead code, no invalid states:

```rust
use ternary_types::Ternary;

/// A traffic light has exactly three states, one for each ternary value.
enum TernaryLight {
    Red,     // -1  — Stop
    Yellow,  //  0  — Caution
    Green,   // +1  — Go
}

impl TernaryLight {
    fn next(&self) -> Self {
        match self {
            // -1 → 0  → +1  → -1 (the Z₃ cycle)
            Self::Red    => Self::Yellow,
            Self::Yellow => Self::Green,
            Self::Green  => Self::Red,
        }
    }
}
```

The Z₃ group ($-1 \rightarrow 0 \rightarrow +1 \rightarrow -1$) is the same cycle as the traffic light. **The math matches the domain exactly.**

## Why This Matters

The bug that binary traffic lights can hit (the invalid `11` state) is real. Real embedded systems have bit flips from radiation, marginal voltage, or timing violations.

Binary's response: add error detection, watchdog timers, and recovery code. More complexity. More testing burden.

Ternary's response: there are only three possible bit patterns for three states. A bit flip in any position produces one of the *other valid states*. The light changes color but never enters an undefined mode.

**The a-ha**: Binary needs 33% more state space than it uses, creating an "error state" that must be handled. Ternary uses 100% of its state space. No redundancy, no dead code, no impossible states.

---

## The Complete Demo

This is exactly what `cargo run --example traffic_controller` in the ternary-cookbook shows:

```rust
use ternary_cookbook::traffic::{Intersection, TimingConfig};

fn main() {
    // A 4-way intersection with three ternary-light directions
    let mut intersection = Intersection::new(TimingConfig::standard());
    
    for cycle in 0..12 {
        println!("Cycle {}: {:?}", cycle, intersection.state());
        intersection.tick();
    }
}
```

The full source is in `examples/traffic_controller.rs`. It handles:
- Three-state lights per direction
- Timing transitions (yellow is shorter than green)
- Emergency override (all directions go to red)
- Pedestrian crossing (interleaving neutral state)

All with no invalid states.

---

## Case Study Takeaway

When your domain has three natural states, don't encode it in binary — you'll create an impossible state you then have to handle. Let the Z₃ group do the work for you.

*Next case study: **Ternary in Practice: The Spam Filter***
