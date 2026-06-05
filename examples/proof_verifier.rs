//! ## Demo 10: Proof Verifier
//!
//! Verifies a chain of assertions where each can be valid (+1), inconclusive (0), or invalid (-1).
//! Real-world application: CI/CD pipeline gate decisions.

use ternary_cookbook::Ternary;

#[derive(Debug)]
struct CheckResult {
    name: &'static str,
    confidence: f64,
}

impl CheckResult {
    fn verdict(&self) -> Ternary {
        if self.confidence >= 0.9 { Ternary::Plus }
        else if self.confidence >= 0.5 { Ternary::Zero }
        else { Ternary::Minus }
    }
}

fn main() {
    println!("=== Ternary Proof Verifier ===\n");
    println!("Simulating a CI/CD pipeline with ternary gate decisions.\n");
    
    // Pipeline: code review → unit tests → integration → security → performance → deploy
    let checks = vec![
        CheckResult { name: "Code Review", confidence: 0.95 },
        CheckResult { name: "Unit Tests", confidence: 0.88 },
        CheckResult { name: "Integration Tests", confidence: 0.92 },
        CheckResult { name: "Security Scan", confidence: 0.45 },
        CheckResult { name: "Performance", confidence: 0.78 },
        CheckResult { name: "Canary Health", confidence: 0.97 },
    ];
    
    let label = |t: Ternary| -> &'static str {
        match t { Ternary::Plus => "✅ PASS", Ternary::Zero => "⚠️ WARN", Ternary::Minus => "❌ FAIL" }
    };
    
    println!("Check            | Confidence | Verdict | Blocks?");
    println!("-----------------|------------|---------|--------");
    
    let mut chain_valid = Ternary::Plus;
    let mut blockers: Vec<&str> = Vec::new();
    let mut warnings: Vec<&str> = Vec::new();
    
    for check in &checks {
        let verdict = check.verdict();
        chain_valid = Ternary::from_i8(chain_valid.to_i8().min(verdict.to_i8()));
        
        let blocks = match verdict {
            Ternary::Minus => { blockers.push(check.name); "YES ⛔" }
            Ternary::Zero => { warnings.push(check.name); "soft ⚠️" }
            Ternary::Plus => "no ✅",
        };
        
        let bar_len = (check.confidence * 20.0) as usize;
        let bar = "█".repeat(bar_len) + &"░".repeat(20 - bar_len);
        
        println!("{:<16} | {:.0}% {} | {} | {}", 
                 check.name, check.confidence * 100.0, &bar[..20], label(verdict), blocks);
    }
    
    println!("\n=== Pipeline Verdict ===");
    println!("Chain result: {}", label(chain_valid));
    
    if !blockers.is_empty() {
        println!("\n⛔ BLOCKERS (must fix before deploy):");
        for b in &blockers { println!("   - {}", b); }
    }
    if !warnings.is_empty() {
        println!("\n⚠️  WARNINGS (should investigate):");
        for w in &warnings { println!("   - {}", w); }
    }
    
    // Demonstrate AND/OR semantics
    println!("\n=== Decision Logic ===");
    let all_must_pass = checks.iter().all(|c| c.verdict() == Ternary::Plus);
    let any_blocking = checks.iter().any(|c| c.verdict() == Ternary::Minus);
    let majority_pass = checks.iter().filter(|c| c.verdict() == Ternary::Plus).count() > checks.len() / 2;
    
    println!("All pass?       {} — strictest gate", if all_must_pass { "✅ YES" } else { "❌ NO" });
    println!("Any blocking?   {} — weakest gate", if any_blocking { "❌ YES" } else { "✅ NO" });
    println!("Majority pass?  {} — democratic gate", if majority_pass { "✅ YES" } else { "❌ NO" });
    
    println!("\n=== Key Takeaway ===");
    println!("CI/CD gates are naturally ternary: PASS/WARN/FAIL.");
    println!("Binary (pass/fail) forces teams to either ignore warnings or block on noise.");
    println!("The WARN state ({:.0}% confidence threshold) captures 'probably fine but check it'", 50.0);
    println!("without blocking the pipeline. Security failures ({:.0}%) are clear blockers.", 
             checks[3].confidence * 100.0);
}
