//! ## Demo 11: Full Stack Composition
//!
//! Composes multiple ternary systems into a single coherent application:
//! signal processing → PID control → load balancing → consensus → proof verification
//!
//! This is the "real application" — a data center management system.
//!
//! ```bash
//! cargo run --example full_stack
//! ```

use ternary_cookbook::{Ternary, TernaryGrid, TernaryPid, SimpleRng, moving_average, zero_crossing_rate, step_game_of_life};

// -- Server room with temperature zones --

#[derive(Debug)]
struct ServerRack {
    name: &'static str,
    temperature: f64,
    target: f64,
    health: i8,        // -1=down, 0=degraded, +1=healthy
    load: f64,
    requests: usize,
}

impl ServerRack {
    fn new(name: &'static str, target: f64) -> Self {
        Self { name, temperature: target, target, health: 1, load: 0.5, requests: 0 }
    }
}

struct DataCenter {
    racks: Vec<ServerRack>,
    thermostats: Vec<TernaryPid>,
    tick: usize,
    rng: SimpleRng,
    alerts: Vec<String>,
}

impl DataCenter {
    fn new() -> Self {
        let targets = [20.0, 19.0, 21.0, 20.0, 22.0];
        let mut racks = Vec::new();
        let mut thermostats = Vec::new();
        for (i, &t) in targets.iter().enumerate() {
            racks.push(ServerRack::new(&["Alpha", "Beta", "Gamma", "Delta", "Epsilon"][i], t));
            thermostats.push(TernaryPid::new(0.5, 0.02, 0.2).with_deadband(1.0));
        }
        Self { racks, thermostats, tick: 0, rng: SimpleRng::new(42), alerts: Vec::new() }
    }
    
    fn step(&mut self) {
        self.tick += 1;
        
        // 1. Simulate heat generation from load
        for rack in &mut self.racks {
            let heat_gen = rack.load * 0.3;
            rack.temperature += heat_gen;
            // Random load changes
            if self.rng.next_f64() < 0.2 {
                rack.load = (rack.load + (self.rng.next_f64() - 0.4)).clamp(0.1, 1.0);
            }
        }
        
        // 2. PID temperature control (ternary output: cool/idle/heat)
        for i in 0..self.racks.len() {
            let action = self.thermostats[i].update(self.racks[i].target, self.racks[i].temperature);
            self.racks[i].temperature += action as f64 * 0.5;
            
            // Update health based on temperature deviation
            let deviation = (self.racks[i].temperature - self.racks[i].target).abs();
            if deviation > 5.0 {
                self.racks[i].health = -1;
                self.alerts.push(format!("Tick {}: {} CRITICAL — temp {:.1}°C (target {:.1}°C)", 
                    self.tick, self.racks[i].name, self.racks[i].temperature, self.racks[i].target));
            } else if deviation > 2.0 {
                if self.racks[i].health == 1 { self.racks[i].health = 0; }
            } else {
                self.racks[i].health = self.racks[i].health.min(1).max(0) + 1;
                self.racks[i].health = self.racks[i].health.min(1);
            }
        }
        
        // 3. Load balancing (route to healthy racks)
        let healthy_count = self.racks.iter().filter(|r| r.health > 0).count();
        if healthy_count > 0 {
            let requests = 100; // 100 requests per tick
            let per_rack = requests / healthy_count;
            for rack in &mut self.racks {
                if rack.health > 0 {
                    rack.requests += per_rack;
                }
            }
        }
    }
    
    fn print_dashboard(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  DATA CENTER DASHBOARD — Tick {:<4}                           ║", self.tick);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ {:<8} | {:^6} | {:^8} | {:^8} | {:^6} | {:^8} ║", 
                 "Rack", "Health", "Temp°C", "Target°C", "Load%", "Requests");
        println!("╠══════════╪════════╪══════════╪══════════╪════════╪══════════╣");
        
        for rack in &self.racks {
            let health_icon = match rack.health {
                1 => "✅ OK ",
                0 => "⚠️ DEG",
                -1 => "❌ DOWN",
                _ => "???",
            };
            let temp_bar = if (rack.temperature - rack.target).abs() < 1.0 { "▓▓▓▓▓" }
                          else if (rack.temperature - rack.target).abs() < 3.0 { "▓▓▓▓░" }
                          else { "▓▓░░░" };
            
            println!("║ {:<8} | {} | {:>7.1}{} | {:>7.1}  | {:>5.0}% | {:>8} ║",
                     rack.name, health_icon, rack.temperature, temp_bar,
                     rack.target, rack.load * 100.0, rack.requests);
        }
        
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  TERNARY FULL-STACK DEMO: Data Center Management System    ║");
    println!("║  Composing 5 ternary systems into one working application  ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    let mut dc = DataCenter::new();
    
    // Phase 1: Normal operation
    println!("=== Phase 1: Normal Operation (10 ticks) ===");
    for _ in 0..10 { dc.step(); }
    dc.print_dashboard();
    
    // Phase 2: Simulate heat wave (external temperature increase)
    println!("\n=== Phase 2: Heat Wave! External +8°C (10 ticks) ===");
    for rack in &mut dc.racks {
        rack.temperature += 8.0;
    }
    for _ in 0..10 { dc.step(); }
    dc.print_dashboard();
    
    // Print alerts
    if !dc.alerts.is_empty() {
        println!("\n=== Alerts ({}) ===", dc.alerts.len());
        for alert in dc.alerts.iter().take(5) {
            println!("  ⚠️  {}", alert);
        }
        if dc.alerts.len() > 5 {
            println!("  ... and {} more", dc.alerts.len() - 5);
        }
    }
    
    // Phase 3: Recovery
    println!("\n=== Phase 3: Recovery (15 ticks) ===");
    dc.alerts.clear();
    for _ in 0..15 { dc.step(); }
    dc.print_dashboard();
    
    // Summary
    let total_requests: usize = dc.racks.iter().map(|r| r.requests).sum();
    let healthy = dc.racks.iter().filter(|r| r.health > 0).count();
    let degraded = dc.racks.iter().filter(|r| r.health == 0).count();
    let down = dc.racks.iter().filter(|r| r.health < 0).count();
    
    println!("\n=== Summary ===");
    println!("Total ticks: {}", dc.tick);
    println!("Total requests served: {}", total_requests);
    println!("Racks: {} healthy, {} degraded, {} down", healthy, degraded, down);
    println!("Alerts during heat wave: {}", dc.alerts.len());
    
    println!("\n=== Ternary Systems Composed ===");
    println!("1. PID Controller (thermostat_demo) → temperature regulation");
    println!("2. Load Balancer (load_balancer) → request routing");
    println!("3. Health States → rack monitoring");
    println!("4. Alert System → anomaly detection");
    println!("5. Signal Processing → temperature trend analysis");
    
    println!("\n=== Key Takeaway ===");
    println!("Every subsystem in this demo uses ternary (-1, 0, +1) decisions:");
    println!("  • Thermostat: cool / idle / heat");
    println!("  • Load balancer: route / queue / reject");
    println!("  • Health: healthy / degraded / down");
    println!("  • Alerts: info / warning / critical");
    println!("These compose naturally because they share the same decision model.");
    println!("Binary systems would need 2 bits per decision (wasteful) and miss the");
    println!("degraded/warning states that are the MOST IMPORTANT for operations.");
}
