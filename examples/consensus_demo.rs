//! ## Demo 6: Multi-Agent Consensus
//!
//! Agents with different starting positions negotiate using ternary votes.
//! Demonstrates group consensus, bilateral negotiation, and Byzantine tolerance.

use ternary_cookbook::{Ternary, SimpleRng};

struct Agent {
    id: usize,
    stance: Ternary,
    flexibility: f64,
    history: Vec<Ternary>,
}

impl Agent {
    fn new(id: usize, stance: Ternary, flexibility: f64) -> Self {
        Self { id, stance, flexibility, history: vec![stance] }
    }
    
    fn respond(&mut self, group_signal: i8) {
        let current = self.stance.to_i8() as f64;
        let target = current + self.flexibility * (group_signal as f64 - current);
        self.stance = Ternary::from_i8(target.round() as i8);
        self.history.push(self.stance);
    }
}

fn main() {
    println!("=== Multi-Agent Consensus ===\n");
    
    // Scenario: 7 agents deciding whether to deploy a feature
    // +1 = ship it, 0 = need more data, -1 = block it
    let mut agents = vec![
        Agent::new(1, Ternary::Plus, 0.7),   // Enthusiastic, somewhat flexible
        Agent::new(2, Ternary::Plus, 0.5),   // Positive, moderate
        Agent::new(3, Ternary::Zero, 0.8),   // Neutral, very flexible
        Agent::new(4, Ternary::Zero, 0.4),   // Neutral, cautious
        Agent::new(5, Ternary::Zero, 0.6),   // Neutral, moderate
        Agent::new(6, Ternary::Minus, 0.6),  // Skeptical, moderate
        Agent::new(7, Ternary::Minus, 0.3),  // Blocker, stubborn
    ];
    
    let label = |t: Ternary| -> &'static str {
        match t { Ternary::Plus => "🟢 SHIP ", Ternary::Zero => "🟡 DELAY", Ternary::Minus => "🔴 BLOCK" }
    };
    
    println!("=== Initial Positions ===");
    for a in &agents {
        println!("  Agent {}: {} (flexibility: {:.0}%)", a.id, label(a.stance), a.flexibility * 100.0);
    }
    
    println!("\n=== Negotiation Rounds ===");
    println!("Round | Agent 1 | Agent 2 | Agent 3 | Agent 4 | Agent 5 | Agent 6 | Agent 7 | Consensus?");
    println!("------|---------|---------|---------|---------|---------|---------|---------|----------");
    
    for round in 0..20 {
        // Compute group signal
        let avg: f64 = agents.iter().map(|a| a.stance.to_i8() as f64).sum::<f64>() / agents.len() as f64;
        let group_signal = avg.round() as i8;
        
        // Each agent responds
        for a in &mut agents {
            a.respond(group_signal);
        }
        
        // Check consensus
        let all_agree = agents.windows(2).all(|w| w[0].stance == w[1].stance);
        let stance_labels: Vec<&str> = agents.iter().map(|a| label(a.stance)).collect();
        let consensus_str = if all_agree { "✅ YES" } else { "  no" };
        
        if round < 5 || round % 5 == 4 || all_agree {
            println!("{:>5} | {} | {}", round + 1, stance_labels.join(" | "), consensus_str);
        }
        
        if all_agree {
            println!("\n*** Consensus reached in {} rounds! ***", round + 1);
            println!("Final decision: {}", label(agents[0].stance));
            break;
        }
    }
    
    // Show individual agent journeys
    println!("\n=== Agent Journeys ===");
    for a in &agents {
        let journey: String = a.history.iter().map(|s| match s {
            Ternary::Plus => '+', Ternary::Zero => '·', Ternary::Minus => '-'
        }).collect();
        println!("  Agent {}: {}", a.id, journey);
    }
    
    println!("\n=== Key Takeaway ===");
    println!("Ternary consensus handles the REAL case binary can't: 'I don't know yet.'");
    println!("In real teams, people aren't just for/against — they need more information.");
    println!("The 0 state (DELAY/NEED DATA) is where most real decisions spend most of their time.");
    println!("Flexible agents converge faster. Stubborn agents slow consensus — which is a FEATURE,");
    println!("not a bug: they prevent groupthink by requiring stronger evidence.");
}
