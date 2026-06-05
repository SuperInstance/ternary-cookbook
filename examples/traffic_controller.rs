//! ## Demo 1: Traffic Light Controller
//!
//! A ternary traffic light where +1 = green, 0 = yellow, -1 = red.
//! Demonstrates state machines, timing, and ternary transitions.
//!
//! ```bash
//! cargo run --example traffic_controller
//! ```

use ternary_cookbook::*;

fn main() {
    println!("=== Ternary Traffic Light Controller ===\n");
    
    let phases: &[(&str, i8, usize)] = &[
        ("North-South", 1, 8),   // Green for 8 ticks
        ("North-South", 0, 2),   // Yellow for 2 ticks
        ("North-South", -1, 2),  // Red for 2 ticks
        ("East-West", 1, 8),     // Green
        ("East-West", 0, 2),     // Yellow
        ("East-West", -1, 2),    // Red
    ];
    
    let mut tick = 0;
    let mut phase_idx = 0;
    let mut phase_tick = 0;
    
    println!("Simulating 30 ticks of traffic control:\n");
    println!("Tick | Direction    | State  | Visual");
    println!("-----|-------------|--------|-------");
    
    for _ in 0..30 {
        let (direction, state, duration) = phases[phase_idx];
        let (visual, label) = match state {
            1 => ("🟢", "GREEN "),
            0 => ("🟡", "YELLOW"),
            -1 => ("🔴", "RED   "),
            _ => ("❓", "???   "),
        };
        
        println!("{:4} | {:11} | {} | {}", tick, direction, label, visual);
        
        tick += 1;
        phase_tick += 1;
        
        if phase_tick >= duration {
            phase_idx = (phase_idx + 1) % phases.len();
            phase_tick = 0;
        }
    }
    
    println!("\n=== Key Takeaway ===");
    println!("Traffic lights are naturally ternary: green (+1), yellow (0), red (-1).");
    println!("The yellow state (0) is the critical safety buffer — it's neither GO nor STOP.");
    println!("In binary, you'd need 2 bits (4 states) to encode 3 states. Ternary is optimal.");
}
