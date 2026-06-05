//! ## Demo 8: Signal Processing Pipeline
//!
//! Processes a ternary signal through filtering, averaging, and analysis.
//! Real-world application: sensor data classification.

use ternary_cookbook::{moving_average, zero_crossing_rate, SimpleRng};

fn main() {
    println!("=== Ternary Signal Processing ===\n");
    
    // Simulate a sensor that reads {-1=below_threshold, 0=normal, +1=above_threshold}
    let mut rng = SimpleRng::new(42);
    
    // Generate a signal with a known pattern: burst of positives, calm, burst of negatives
    let mut signal: Vec<i8> = Vec::new();
    for _ in 0..20 { signal.push(0); }       // Calm baseline
    for _ in 0..15 { signal.push(1); }        // Spike up (anomaly)
    for _ in 0..10 { signal.push(0); }        // Return to normal
    for _ in 0..15 { signal.push(-1); }       // Spike down (anomaly)
    for _ in 0..20 { signal.push(0); }        // Calm again
    for _ in 0..10 { signal.push(1); }        // Brief spike
    for _ in 0..10 { signal.push(0); }        // Calm
    
    // Add some noise
    for s in &mut signal {
        if rng.next_f64() < 0.1 { *s = rng.next_i8(); }
    }
    
    println!("Raw signal ({} samples):", signal.len());
    let visual: String = signal.iter().map(|&v| match v {
        1 => '▲', 0 => '─', -1 => '▼', _ => '?'
    }).collect();
    println!("{}\n", visual);
    
    // Moving average (window=5)
    let ma = moving_average(&signal, 5);
    println!("Moving average (window=5):");
    let ma_visual: String = ma.iter().map(|&v| {
        if v > 0.3 { '▲' } else if v < -0.3 { '▼' } else { '─' }
    }).collect();
    println!("{}\n", ma_visual);
    
    // Zero crossing rate
    let zcr = zero_crossing_rate(&signal);
    println!("Zero crossing rate: {:.3}", zcr);
    
    // Signal statistics
    let pos_count = signal.iter().filter(|&&v| v == 1).count();
    let neg_count = signal.iter().filter(|&&v| v == -1).count();
    let zero_count = signal.iter().filter(|&&v| v == 0).count();
    
    println!("\nDistribution: ▲={} | ─={} | ▼={}", pos_count, zero_count, neg_count);
    
    // Anomaly detection: segments where signal is consistently non-zero
    println!("\n=== Anomaly Detection ===");
    let mut in_anomaly = false;
    let mut anomaly_start = 0;
    let mut anomalies: Vec<(usize, usize, i8)> = Vec::new();
    
    for (i, &v) in signal.iter().enumerate() {
        if !in_anomaly && v != 0 {
            in_anomaly = true;
            anomaly_start = i;
        } else if in_anomaly && v == 0 {
            in_anomaly = false;
            let dominant = if signal[anomaly_start..i].iter().filter(|&&v| v == 1).count() > 
                              signal[anomaly_start..i].iter().filter(|&&v| v == -1).count() { 1 } else { -1 };
            anomalies.push((anomaly_start, i - 1, dominant));
        }
    }
    
    for (start, end, direction) in &anomalies {
        let dir_label = if *direction == 1 { "SPIKE UP  ▲" } else { "SPIKE DOWN ▼" };
        println!("  Samples {:3}-{:<3} ({} samples): {}", start, end, end - start + 1, dir_label);
    }
    
    println!("\n=== Key Takeaway ===");
    println!("Ternary sensors are common in industry: pressure switches (low/normal/high),");
    println!("vibration monitors (below/normal/above threshold), quality control (fail/warn/pass).");
    println!("Moving average smooths noise. Zero crossing rate measures signal volatility.");
    println!("The 0 state (normal) is usually the majority — compression is efficient because");
    println!("long runs of 0 compress to almost nothing.");
}
