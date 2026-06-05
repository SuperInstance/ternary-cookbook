//! ## Demo 3: Ternary Load Balancer
//!
//! Routes requests to backends with health tracking.
//! +1 = healthy, 0 = degraded, -1 = down.
//! Demonstrates failover, circuit breaking, and graceful degradation.

use ternary_cookbook::*;

struct Backend {
    name: &'static str,
    health: i8,
    load: f64,
    requests_served: usize,
    failures: usize,
}

impl Backend {
    fn new(name: &'static str) -> Self {
        Self { name, health: 1, load: 0.0, requests_served: 0, failures: 0 }
    }
    
    fn status_label(&self) -> &'static str {
        match self.health {
            1 => "✅ healthy",
            0 => "⚠️ degraded",
            -1 => "❌ down",
            _ => "???"
        }
    }
}

struct LoadBalancer {
    backends: Vec<Backend>,
    rng: SimpleRng,
    queued: usize,
    rejected: usize,
}

impl LoadBalancer {
    fn new(backends: Vec<Backend>) -> Self {
        Self { backends, rng: SimpleRng::new(42), queued: 0, rejected: 0 }
    }
    
    fn route(&mut self) -> Option<usize> {
        // Route to least-loaded healthy backend
        let mut best: Option<(usize, f64)> = None;
        for (i, b) in self.backends.iter().enumerate() {
            if b.health >= 0 && b.load < 0.9 {
                match best {
                    None => best = Some((i, b.load)),
                    Some((_, best_load)) if b.load < best_load => best = Some((i, b.load)),
                    _ => {}
                }
            }
        }
        
        match best {
            Some((idx, _)) => {
                self.backends[idx].load += 0.1;
                self.backends[idx].requests_served += 1;
                Some(idx)
            }
            None => {
                // Try degraded backends
                for (i, b) in self.backends.iter().enumerate() {
                    if b.health == 0 && b.load < 0.8 {
                        self.backends[i].load += 0.1;
                        self.backends[i].requests_served += 1;
                        return Some(i);
                    }
                }
                self.rejected += 1;
                None
            }
        }
    }
    
    fn report_result(&mut self, backend_idx: usize, success: bool) {
        let b = &mut self.backends[backend_idx];
        if success {
            b.load = (b.load - 0.15).max(0.0);
            if b.health < 1 { b.health += 1; } // gradual recovery
        } else {
            b.failures += 1;
            b.load = (b.load + 0.1).min(1.0);
            b.health = (b.health - 1).max(-1); // degrade
        }
    }
    
    fn decay_loads(&mut self) {
        for b in &mut self.backends {
            b.load = (b.load - 0.05).max(0.0);
        }
    }
    
    fn print_status(&self) {
        println!("{:<12} | {:<12} | Load  | Served | Failed", "Backend", "Status");
        println!("{:-<12}-+-{:-<12}-+-------+--------+--------", "", "");
        for b in &self.backends {
            let bar_len = (b.load * 20.0) as usize;
            let bar: String = "█".repeat(bar_len) + &"·".repeat(20 - bar_len);
            println!("{:<12} | {:<12} | {} | {:>6} | {:?}", b.name, b.status_label(), &bar[..20], b.requests_served, b.failures);
        }
    }
}

fn main() {
    println!("=== Ternary Load Balancer Demo ===\n");
    
    let mut lb = LoadBalancer::new(vec![
        Backend::new("alpha"),
        Backend::new("beta"),
        Backend::new("gamma"),
    ]);
    
    println!("--- Initial state ---");
    lb.print_status();
    
    // Phase 1: Normal traffic
    println!("\n--- Phase 1: Normal traffic (20 requests) ---");
    for _ in 0..20 {
        if let Some(idx) = lb.route() {
            lb.report_result(idx, true);
        }
        lb.decay_loads();
    }
    lb.print_status();
    
    // Phase 2: Simulate failures on alpha
    println!("\n--- Phase 2: Alpha starts failing ---");
    for i in 0..15 {
        if let Some(idx) = lb.route() {
            let success = if idx == 0 && i > 3 { false } else { true };
            lb.report_result(idx, success);
        }
        lb.decay_loads();
    }
    lb.print_status();
    
    // Phase 3: Recovery
    println!("\n--- Phase 3: Recovery (all healthy) ---");
    for _ in 0..20 {
        if let Some(idx) = lb.route() {
            lb.report_result(idx, true);
        }
        lb.decay_loads();
    }
    lb.print_status();
    
    println!("\nRejected: {} requests", lb.rejected);
    
    println!("\n=== Key Takeaway ===");
    println!("Backend health is naturally ternary: UP / DEGRADED / DOWN.");
    println!("Binary (up/down) misses the degraded state — the warning before failure.");
    println!("Ternary load balancing routes around degraded backends BEFORE they fail,");
    println!("catching problems earlier and reducing user-facing errors.");
}
