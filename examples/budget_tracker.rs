use ternary_cookbook::SimpleRng;

fn main() {
    println!("=== Ternary Budget Tracker ===\n");
    
    let mut rng = SimpleRng::new(42);
    
    let categories = ["Engineering", "Marketing", "Sales", "Operations", "R&D"];
    let budgets = [50000.0, 30000.0, 40000.0, 20000.0, 60000.0];
    let mut spent = [0.0; 5];
    
    // Simulate 12 months of spending
    println!("Monthly budget status: █ under budget · on track ░ over budget\n");
    println!("{:<12} | Budget   | Spent    | Status", "Category");
    println!("{:-<12}-+-{:-<9}-+-{:-<9}-+--------", "", "", "");
    
    let mut total_budget = 0.0;
    let mut total_spent = 0.0;
    
    for (i, cat) in categories.iter().enumerate() {
        // Simulate spending with variance
        for _ in 0..12 {
            let monthly = budgets[i] / 12.0;
            let variance = (rng.next_f64() - 0.3) * monthly * 0.5;
            spent[i] += monthly + variance;
        }
        
        total_budget += budgets[i];
        total_spent += spent[i];
        
        let ratio = spent[i] / budgets[i];
        let (status, icon) = if ratio > 1.1 { ("OVER BUDGET", "🔴 ░░░") }
                            else if ratio < 0.9 { ("UNDER BUDGET", "🟢 ███") }
                            else { ("ON TRACK", "🟡 █░░") };
        
        println!("{:<12} | {:>7.0}  | {:>7.0}  | {} {}", cat, budgets[i], spent[i], icon, status);
    }
    
    println!("{:-<12}-+-{:-<9}-+-{:-<9}-+--------", "", "", "");
    let overall_ratio = total_spent / total_budget;
    let overall_status = if overall_ratio > 1.1 { "🔴 OVER" } else if overall_ratio < 0.9 { "🟢 UNDER" } else { "🟡 ON TRACK" };
    println!("{:<12} | {:>7.0}  | {:>7.0}  | {} ({:.1}%)", "TOTAL", total_budget, total_spent, overall_status, overall_ratio * 100.0);
    
    println!("\n=== Key Takeaway ===");
    println!("Budget tracking is ternary: UNDER / ON TRACK / OVER.");
    println!("Binary (over/under) misses the 'exactly on budget' case — which is the GOAL.");
    println!("The on-track zone (90-110%) is the target. Under-budget isn't always good");
    println!("(might mean under-investing). Ternary captures this nuance naturally.");
}
