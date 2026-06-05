//! ## Demo 5: PID Thermostat
//!
//! Ternary PID controller: output is -1=cool, 0=idle, +1=heat.
//! Demonstrates deadband, anti-windup, and settling behavior.

use ternary_cookbook::TernaryPid;

fn main() {
    println!("=== Ternary PID Thermostat ===\n");
    
    let target_temp = 22.0;
    let mut pid = TernaryPid::new(0.8, 0.05, 0.3).with_deadband(0.5);
    let mut current_temp = 15.0; // Start cold
    let mut history: Vec<(usize, f64, i8)> = Vec::new();
    
    println!("Target: {:.1}°C | Starting: {:.1}°C\n", target_temp, current_temp);
    println!("Tick | Current°C | Action  | Error°C | Output");
    println!("-----|-----------|---------|---------|-------");
    
    for tick in 0..60 {
        let action = pid.update(target_temp, current_temp);
        let error = target_temp - current_temp;
        
        // Simulate heating/cooling effect
        current_temp += action as f64 * 0.3;
        // Add slight noise
        current_temp += ((tick * 7 + 3) as f64 * 0.001 - 0.0005);
        
        let action_label = match action {
            1 => "🔥 HEAT ",
            0 => "💤 IDLE ",
            -1 => "❄️  COOL",
            _ => "???     ",
        };
        
        if tick % 5 == 0 || tick < 10 {
            println!("{:>4} | {:>7.2}°C | {} | {:>+6.2}°C | {}", 
                     tick, current_temp, action_label, error, 
                     "█".repeat((current_temp as usize).min(30)));
        }
        
        history.push((tick, current_temp, action));
    }
    
    // Efficiency analysis
    let idle_ticks = history.iter().filter(|&&(_, _, a)| a == 0).count();
    let efficiency = idle_ticks as f64 / history.len() as f64;
    
    println!("\n=== Efficiency Report ===");
    println!("Total ticks: {}", history.len());
    println!("Heating ticks: {}", history.iter().filter(|&&(_, _, a)| a == 1).count());
    println!("Cooling ticks: {}", history.iter().filter(|&&(_, _, a)| a == -1).count());
    println!("Idle ticks: {} ({:.0}% efficient)", idle_ticks, efficiency * 100.0);
    println!("Final temperature: {:.2}°C (target: {:.1}°C)", current_temp, target_temp);
    
    println!("\n=== Key Takeaway ===");
    println!("The ternary output (-1, 0, +1) is what real HVAC systems actually DO.");
    println!("You can't 'heat at 73% duty cycle' — you either run the heater or don't.");
    println!("The deadband (0.5°C) prevents chattering. The PID loop naturally settles to idle.");
    println!("Idle percentage is the efficiency metric — a good ternary controller maximizes idle time.");
}
