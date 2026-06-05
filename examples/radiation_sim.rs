use ternary_cookbook::{TernaryGrid, SimpleRng};

fn main() {
    println!("=== Ternary Radiation Damage Simulation ===\n");
    
    let mut rng = SimpleRng::new(42);
    let size = 50;
    let mut grid = TernaryGrid::new(size, size, 1); // All intact
    
    println!("Simulating radiation damage on a {}x{} material lattice.", size, size);
    println!("States: █ intact (+1) · damaged (0) ░ destroyed (-1)\n");
    
    // Initial state
    println!("Generation 0 (100% intact):");
    println!("{}", grid.render());
    
    // Simulate 10 rounds of irradiation + annealing
    for gen in 1..=10 {
        let dose_rate = 0.02 + gen as f64 * 0.005; // Increasing dose
        let temperature = 3.0; // Annealing temperature
        
        // Irradiate: randomly damage cells
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) == 1 && rng.next_f64() < dose_rate {
                    grid.set(x, y, -1); // Direct hit
                    // Cascade: damage neighbors
                    for (nx, ny) in grid.neighbors4(x, y) {
                        if grid.get(nx, ny) == 1 && rng.next_f64() < 0.3 {
                            grid.set(nx, ny, 0);
                        }
                    }
                }
            }
        }
        
        // Anneal: damaged cells near intact cells can recover
        let mut recovery = vec![0i8; size * size];
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) < 1 {
                    let intact_neighbors = grid.neighbors4(x, y).iter()
                        .filter(|&&(nx, ny)| grid.get(nx, ny) == 1).count();
                    if intact_neighbors >= 2 && rng.next_f64() < 0.3 {
                        recovery[y * size + x] = 1;
                    }
                }
            }
        }
        for y in 0..grid.height {
            for x in 0..grid.width {
                if recovery[y * size + x] == 1 { grid.set(x, y, 1); }
            }
        }
        
        let intact = grid.count(1);
        let damaged = grid.count(0);
        let destroyed = grid.count(-1);
        let integrity = intact as f64 / (size * size) as f64 * 100.0;
        
        if gen % 3 == 0 || gen == 10 {
            println!("Generation {} (integrity: {:.1}%):", gen, integrity);
            println!("{}", grid.render());
        }
    }
    
    let final_intact = grid.count(1);
    let final_integrity = final_intact as f64 / (size * size) as f64;
    
    println!("\n=== Damage Report ===");
    println!("Initial integrity: 100.0%");
    println!("Final integrity:   {:.1}%", final_integrity * 100.0);
    println!("Total damage:      {:.1}%", (1.0 - final_integrity) * 100.0);
    
    println!("\n=== Key Takeaway ===");
    println!("Material states are ternary: INTACT / DAMAGED / DESTROYED.");
    println!("Binary (intact/destroyed) misses partially damaged material that can still");
    println!("be annealed back to health. The 0 state (damaged but recoverable) is critical");
    println!("for deciding whether to repair or replace — a binary model would waste resources");
    println!("replacing recoverable material or leaving damage to spread.");
}
