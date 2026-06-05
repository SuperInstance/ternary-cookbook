//! ## Demo 4: Ternary Game of Life
//!
//! Three-state cellular automaton where cells can be young (+1), dead (0), or old (-1).
//! Demonstrates cyclic boom/bust with a natural lifecycle clock.
//!
//! ```bash
//! cargo run --example game_of_life
//! ```

use ternary_cookbook::{TernaryGrid, step_game_of_life, SimpleRng};

fn main() {
    println!("=== Ternary Game of Life ===\n");
    println!("Legend: █ = young (+1)  · = dead (0)  ░ = old (-1)\n");
    
    let mut rng = SimpleRng::new(42);
    let mut grid = TernaryGrid::new(30, 15, 0);
    
    // Seed with random pattern (40% young, 20% old)
    for y in 0..grid.height {
        for x in 0..grid.width {
            let r = rng.next_f64();
            if r < 0.3 { grid.set(x, y, 1); }
            else if r < 0.4 { grid.set(x, y, -1); }
        }
    }
    
    let generations = 40;
    
    println!("Generation 0:");
    println!("{}", grid.render());
    
    let mut pop_history: Vec<(usize, usize, usize)> = Vec::new();
    
    for gen in 1..=generations {
        grid = step_game_of_life(&grid);
        
        let young = grid.count(1);
        let dead = grid.count(0);
        let old = grid.count(-1);
        pop_history.push((young, dead, old));
        
        if gen % 10 == 0 || gen == generations {
            println!("Generation {} (young:{} dead:{} old:{}):", gen, young, dead, old);
            println!("{}", grid.render());
        }
    }
    
    // Print population dynamics
    println!("\n=== Population Dynamics ===");
    println!("Gen | Young | Dead  | Old   | Total");
    println!("----|-------|-------|-------|------");
    for (i, (young, dead, old)) in pop_history.iter().enumerate() {
        if i % 5 == 0 {
            let bar_y: String = "█".repeat((*young as f64 / 10.0) as usize);
            println!("{:>3} | {:>5} | {:>5} | {:>5} | {:>5} {}", i + 1, young, dead, old, young + dead + old, bar_y);
        }
    }
    
    println!("\n=== Key Takeaway ===");
    println!("The three-state lifecycle {{young, dead, old}} creates a BUILT-IN CLOCK.");
    println!("Booms and busts cycle naturally — unlike binary GoL which has no aging.");
    println!("The old state (-1) acts as a refractory period, preventing immediate rebirth.");
    println!("This is analogous to neuron refractory periods and forest succession cycles.");
}
