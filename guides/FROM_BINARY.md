# From Binary to Ternary

*A migration guide for developers who know binary systems and want to understand why ternary is different — and why it matters.*

---

## The Trap You Don't Know You're In

Every programmer learns early: computers are binary. Ones and zeros. True and false. On and off.

This becomes a habit. When you design a system, you reach for booleans. If you need more states, you add more booleans — flags, bitmasks, enums flattened into bits.

**The trap**: binary is a *hardware* constraint. It's not a *logic* constraint. Your problems aren't binary; your transistors are.

Ternary takes the opposite approach: start with the logic your problem actually needs, then encode it.

---

## Map Your Domain to the Three States

Every domain has a natural three-state rhythm. The first step in migration is finding it.

| Your domain | -1 | 0 | +1 |
|-------------|----|---|-----|
| Access control | Deny | Escalate | Allow |
| Server monitoring | Down | Degraded | Healthy |
| CI/CD | Fail | Warnings | Pass |
| Anomaly detection | Anomaly | Uncertain | Normal |
| Moderation | Reject | Review | Approve |
| Routing | Blocked | Pending | Connected |

Identify your three states before you write a single line of code. This is the migration's foundation.

---

## Pattern: From Boolean Flags to Ternary State

**Before (binary):**
```rust
struct ServerStatus {
    is_up: bool,           // alive or dead
    is_overloaded: bool,   // true = bad, false = fine
}
// Four possible states, but two are contradictory:
// (true, true)  — up but overloaded? makes sense
// (true, false) — up and fine
// (false, true) — dead but overloaded? meaningless
// (false, false) — dead and not overloaded
```

Three of four combinations might be valid, but there's no way to express "partially up" or "restarting." You're forced to either add more flags or accept the information gap.

**After (ternary):**
```rust
use ternary_types::Ternary;

struct ServerStatus {
    health: Ternary,  // -1 = down, 0 = degraded, +1 = healthy
}
// Three states, zero contradictions.
// "Degraded" is a real thing servers do.
```

**The a-ha**: Booleans create impossible states. Ternary aligns with reality.

---

## Pattern: From If-Else Chains to Ternary Composition

**Before (binary):**
```rust
fn check_access(user: &User, resource: &str) -> Decision {
    if user.is_banned { return Decision::Deny; }
    if user.role == "admin" { return Decision::Allow; }
    if resource.starts_with("/admin") { return Decision::Deny; }
    if time_of_day > 17 { return Decision::Pending; }
    // ...
    Decision::Allow
}
```

Linear chain. Order matters. Adding a rule means figuring out where it goes in the priority stack.

**After (ternary):**
```rust
use ternary_types::{Ternary, TernaryOps};

fn check_access(user: &User, resource: &str) -> Ternary {
    // Each rule returns a ternary value; they compose algebraically
    let role_check = match user.role {
        "admin" => Ternary::Positive,
        "banned" => Ternary::Negative,
        _ => Ternary::Neutral,  // don't know yet
    };
    let resource_check = if resource.starts_with("/admin") {
        Ternary::Negative
    } else {
        Ternary::Neutral
    };
    let time_check = if time_of_day > 17 {
        Ternary::Neutral  // needs escalation
    } else {
        Ternary::Positive
    };
    
    // Compose: role has veto power, resources constrain, time adjusts
    role_check * resource_check * time_check
}
```

**The a-ha**: Order doesn't matter. Each rule contributes its vote, and the ternary algebra combines them. Add rules anywhere, remove them anywhere — the math handles it.

---

## Pattern: From Error Handling to Ternary Signals

**Before (binary):**
```rust
Result<T, E>  // Success or failure
// But what about "success with warnings"?
// What about "partial success"?
```

**After (ternary):**
```rust
enum TernaryResult<T> {
    Success(T),       // +1: everything worked
    Partial(T, Vec<Warning>),  // 0: success with caveats
    Failure(Error),   // -1: something broke
}
```

**The a-ha**: Real systems don't just succeed or fail. They partially succeed *all the time*. Ternary gives you a vocabulary for that.

---

## Pattern: From State Machine to Ternary Dynamics

**Before (binary):**
A traffic light needs three states. Binary state machines use two bits and one invalid state:

```rust
// Two bits for three states = 25% state space wasted
enum TrafficLight {
    Red,     // 00
    Yellow,  // 01
    Green,   // 10
    // 11 is invalid — what happens if you hit it?
}
```

**After (ternary):**
```rust
use ternary_types::Ternary;

// Three states, 100% utilization
enum TrafficLight {
    Red,     // -1
    Yellow,  //  0
    Green,   // +1
}
```

**The a-ha**: Ternary state machines have no dead states and no invalid transitions. The Z₃ cycle (−1 → 0 → +1 → −1) maps naturally to cyclical systems.

Run it: `cargo run --example traffic_controller`

---

## Pattern: From PID to Ternary Control

**Before:** A PID controller uses floating point math with accumulation errors, integral windup, and derivative noise.

**After:** `ternary-pid` maps the control surface to three discrete states. No floating point, no windup, no noise amplification.

Run it: `cargo run --example thermostat_demo`

**The a-ha**: Discrete control with ternary states can stabilize systems that floating-point PID fights. The 0 state acts as a deadband — if the system is close enough to target, it does nothing. This prevents oscillation.

---

## Migration Checklist

- [ ] Identify your domain's three natural states
- [ ] Replace boolean flags with `Ternary`
- [ ] Replace if-else chains with ternary composition (`+`, `*`)
- [ ] Check for conservation: sum of your system's states should be meaningful
- [ ] Add "See Also" links to symmetry siblings in the ecosystem
- [ ] Run Forgemaster's cookbook demos to verify understanding

---

## What You're Now Ready For

| Cookbook demo | What it teaches |
|---------------|-----------------|
| `traffic_controller` | Cyclical state machines |
| `spam_filter` | Classification with deadband |
| `load_balancer` | Health-aware routing |
| `thermostat_demo` | PID control without floating point |
| `consensus_demo` | Multi-agent agreement with three outcomes |
| `full_stack` | All patterns composed into one system |

The ecosystem is 250 crates deep and every one shares the same $\{-1, 0, +1\}$ vocabulary. You now speak the language.
