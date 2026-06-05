# Ternary Developer Guide

## A Practical Introduction to {-1, 0, +1} Systems

This guide teaches you how to think in ternary and build real applications using the SuperInstance ternary ecosystem.

---

## Table of Contents

1. [Why Ternary?](#why-ternary)
2. [Quick Start](#quick-start)
3. [Core Concepts](#core-concepts)
4. [Pattern: Ternary State Machines](#pattern-state-machines)
5. [Pattern: Ternary Classifiers](#pattern-classifiers)
6. [Pattern: Ternary Control Systems](#pattern-control)
7. [Pattern: Ternary Consensus](#pattern-consensus)
8. [Anti-Patterns to Avoid](#anti-patterns)
9. [When NOT to Use Ternary](#when-not)
10. [Composing Multiple Systems](#composition)

---

## Why Ternary? <a name="why-ternary"></a>

**The short answer**: because most real decisions have three outcomes, not two.

Binary forces you into `true/false`, `pass/fail`, `up/down`. But real systems have a middle state:

| Domain | -1 | 0 | +1 |
|--------|----|---|-----|
| Email filtering | Spam | Review | Ham |
| Server health | Down | Degraded | Healthy |
| Trading | Sell | Hold | Buy |
| CI/CD | Fail | Warning | Pass |
| Thermostat | Cool | Idle | Heat |
| Budget | Over | On-track | Under |

**The 0 state is the most important one.** It captures:
- "I don't know yet" (classification)
- "Everything's fine" (monitoring)
- "No action needed" (control)
- "Abstain" (voting)

Our research discovered that **the 0 state is a universal screen** — it prevents pathological lock-in in every system we tested. Binary systems that lack the 0 state are vulnerable to oscillation, monoculture, and cascade failures.

---

## Quick Start <a name="quick-start"></a>

```bash
# Clone the cookbook
git clone https://github.com/SuperInstance/ternary-cookbook.git
cd ternary-cookbook

# Run any example
cargo run --example traffic_controller    # Traffic light state machine
cargo run --example spam_filter           # Email classification
cargo run --example load_balancer         # Health-aware routing
cargo run --example thermostat_demo       # PID climate control
cargo run --example consensus_demo        # Multi-agent voting
cargo run --example full_stack            # Composed system (the big one)
```

---

## Core Concepts <a name="core-concepts"></a>

### The Three States

```rust
// Every ternary value is one of:
-1  // Negative, reject, below, inactive, down, sell, fail
 0  // Neutral, abstain, on-target, idle, degraded, hold, warn
+1  // Positive, accept, above, active, healthy, buy, pass
```

### Ternary AND/OR

```rust
// AND: minimum (pessimistic combining)
fn ternary_and(a: i8, b: i8) -> i8 { a.min(b) }
// +1 AND  0 =  0  (positive + neutral = neutral)
// +1 AND -1 = -1  (positive + negative = negative)
//  0 AND  0 =  0  (neutral + neutral = neutral)

// OR: maximum (optimistic combining)
fn ternary_or(a: i8, b: i8) -> i8 { a.max(b) }
// +1 OR  -1 = +1  (at least one positive = positive)
//  0 OR  -1 =  0  (neutral is better than negative)
```

### The Z₃ Group

Ternary addition mod 3 is the only group structure on three elements:

```
-1 + -1 = +1  (mod 3)
-1 +  0 = -1
-1 + +1 =  0
 0 +  0 =  0
+1 + +1 = -1  (mod 3)
```

This cyclic structure is what drives rock-paper-scissors dynamics and makes Z₃ self-balancing.

---

## Pattern: Ternary State Machines <a name="pattern-state-machines"></a>

**Use when**: Your system transitions between three states cyclically.

```rust
// Example: Traffic light
enum TrafficState { Green, Yellow, Red }

impl TrafficState {
    fn to_ternary(&self) -> i8 {
        match self {
            TrafficState::Green => 1,
            TrafficState::Yellow => 0,   // The transition state!
            TrafficState::Red => -1,
        }
    }
}
```

**Why ternary is better**: Binary would need 2 bits (4 states, one wasted) and has no natural transition state. The yellow light IS the 0 state — it's neither go nor stop.

**See**: `cargo run --example traffic_controller`

---

## Pattern: Ternary Classifiers <a name="pattern-classifiers"></a>

**Use when**: You need to classify inputs with an "I don't know" option.

```rust
fn classify(input: &Data, threshold: f64) -> i8 {
    let score = compute_score(input);
    if score > threshold { 1 }          // Positive
    else if score < -threshold { -1 }   // Negative
    else { 0 }                           // Unsure — send to review
}
```

The `threshold` controls the **deadband width**:
- Wide deadband → fewer false positives, more "unsure"
- Narrow deadband → fewer "unsure", more false positives

**Real application**: The spam filter (`cargo run --example spam_filter`) classifies emails as spam/unsure/ham with adjustable threshold.

---

## Pattern: Ternary Control Systems <a name="pattern-control"></a>

**Use when**: You're controlling something that can go positive, negative, or stay idle.

```rust
let mut pid = TernaryPid::new(kp, ki, kd).with_deadband(0.5);
loop {
    let action = pid.update(target, current_measurement);
    match action {
        1 => heat_up(),
        0 => do_nothing(),   // Within deadband — most efficient state!
        -1 => cool_down(),
    }
}
```

**Efficiency metric**: Track what fraction of time is spent in state 0. Higher = more efficient.

**See**: `cargo run --example thermostat_demo`

---

## Pattern: Ternary Consensus <a name="pattern-consensus"></a>

**Use when**: Multiple agents need to agree, and "I need more information" is a valid position.

```rust
struct Agent {
    stance: i8,          // Current vote
    flexibility: f64,    // How much they move toward group average
}

impl Agent {
    fn respond(&mut self, group_signal: i8) {
        let target = self.stance as f64 
            + self.flexibility * (group_signal as f64 - self.stance as f64);
        self.stance = target.round() as i8;
    }
}
```

**Key insight**: Stubborn agents (low flexibility) prevent groupthink. Flexible agents converge faster. You need both.

**See**: `cargo run --example consensus_demo`

---

## Anti-Patterns to Avoid <a name="anti-patterns"></a>

### ❌ Treating ternary as "binary with a bonus state"
The 0 state isn't a fallback — it's the PRIMARY state in healthy systems. Most cells in a ternary grid should be 0 most of the time.

### ❌ Ignoring the deadband
Without a deadband, ternary systems chatter (oscillate rapidly between +1 and -1). Always include a deadband around 0.

### ❌ Using +1 and -1 symmetrically
In many real systems, the states aren't symmetric. "Heat" and "cool" have different costs. Weight your PID gains accordingly.

### ❌ Forgetting that 0 screens everything
Our research showed: the 0 state prevents synchronization, phase transitions, and long-range order. If you remove it, your system WILL lock into monoculture.

---

## When NOT to Use Ternary <a name="when-not"></a>

- **You need more than 3 states**: Use multi-valued logic or enums
- **Your data is continuous**: Use floats, not ternary
- **You need precise proportional control**: Ternary is bang-bang control
- **You have no natural "neutral" state**: If every input must be positive or negative, binary is fine

---

## Composing Multiple Systems <a name="composition"></a>

The real power of ternary: **all ternary systems compose naturally** because they share the same {-1, 0, +1} decision model.

```rust
// Temperature control → health assessment → load balancing → routing
let temp_action = thermostat.update(target, current_temp);   // {-1, 0, +1}
let health = assess_health(temp_action, deviation);          // {-1, 0, +1}
let route_decision = load_balancer.route(health);            // {-1, 0, +1}
```

No conversion needed between stages. Every subsystem speaks the same language.

**See the full composition**: `cargo run --example full_stack`

---

## Research Foundations

This cookbook is based on research published across 200+ ternary crates and 4,300+ tests:

1. **Z₃ is the only algebraic group on ternary** — cyclic addition mod 3
2. **The 0 state is a universal screen** — prevents synchronization, phase transitions, and monoculture
3. **RPS dynamics maintain biodiversity** — cyclic dominance is self-balancing
4. **No ternary phase transition exists** — ternary systems exist in a single phase (unlike binary Ising)
5. **Forgiveness is costly, trust is beneficial** — from grace vs trust 2D sweeps

---

## License

MIT
