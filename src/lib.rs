//! # Ternary Cookbook: Working Demos and Developer Guides
//!
//! This cookbook contains **11 runnable examples** that demonstrate real-world
//! applications of ternary {-1, 0, +1} logic. Each example is self-contained
//! and can be run with `cargo run --example <name>`.
//!
//! ## Quick Start
//!
//! ```bash
//! cargo run --example traffic_controller    # Simulated traffic light controller
//! cargo run --example spam_filter           # Ternary spam classification
//! cargo run --example load_balancer         # Health-aware load balancer
//! cargo run --example game_of_life          # Ternary Game of Life
//! cargo run --example thermostat_demo       # PID climate control
//! cargo run --example consensus_demo        # Multi-agent voting
//! cargo run --example budget_tracker        # Resource allocation
//! cargo run --example signal_processor      # Ternary DSP pipeline
//! cargo run --example radiation_sim         # Radiation damage simulation
//! cargo run --example proof_verifier        # Proof chain verification
//! cargo run --example full_stack            # All systems composed together
//! ```
//!
//! ## The Ternary Model
//!
//! In balanced ternary, every value is one of:
//! - **`+1`** — positive, active, accept, above
//! - **`0`** — neutral, idle, abstain, on-target  
//! - **`-1`** — negative, inactive, reject, below
//!
//! This maps naturally to real decisions:
//! - Buy / Hold / Sell
//! - Approve / Review / Reject
//! - Heat / Idle / Cool
//! - Healthy / Degraded / Down
//!
//! The key insight from our research: **the 0 state is a universal screen**.
//! It prevents pathological lock-in, screens phase transitions, and makes
//! Z₃ cyclic dominance (rock-paper-scissors) the only natural coordination mechanism.

// ===== Core Types (shared across examples) =====

/// Core ternary value
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ternary {
    Minus,
    Zero,
    Plus,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Self {
        match v { -1 => Ternary::Minus, 0 => Ternary::Zero, 1 => Ternary::Plus, _ => Ternary::Zero }
    }
    pub fn to_i8(self) -> i8 { match self { Ternary::Minus => -1, Ternary::Zero => 0, Ternary::Plus => 1 } }
    pub fn label(self) -> &'static str {
        match self { Ternary::Minus => "[-]", Ternary::Zero => "[ ]", Ternary::Plus => "[+]" }
    }
}

// ===== Ternary Grid =====

/// A 2D grid of ternary values
#[derive(Clone)]
pub struct TernaryGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<i8>,
}

impl TernaryGrid {
    pub fn new(width: usize, height: usize, fill: i8) -> Self {
        Self { width, height, cells: vec![fill; width * height] }
    }
    pub fn get(&self, x: usize, y: usize) -> i8 { self.cells[y * self.width + x] }
    pub fn set(&mut self, x: usize, y: usize, v: i8) { self.cells[y * self.width + x] = v.clamp(-1, 1); }
    
    pub fn count(&self, state: i8) -> usize {
        self.cells.iter().filter(|&&v| v == state).count()
    }
    
    /// Render as ASCII art
    pub fn render(&self) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get(x, y) {
                    -1 => '░',
                    0 => '·',
                    1 => '█',
                    _ => '?',
                };
                out.push(c);
            }
            out.push('\n');
        }
        out
    }
    
    /// Neighbors (4-connectivity)
    pub fn neighbors4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if x > 0 { n.push((x-1, y)); }
        if x < self.width-1 { n.push((x+1, y)); }
        if y > 0 { n.push((x, y-1)); }
        if y < self.height-1 { n.push((x, y+1)); }
        n
    }
}

// ===== PID Controller =====

pub struct TernaryPid {
    kp: f64, ki: f64, kd: f64,
    integral: f64,
    prev_error: f64,
    deadband: f64,
    initialized: bool,
}

impl TernaryPid {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self { kp, ki, kd, integral: 0.0, prev_error: 0.0, deadband: 0.0, initialized: false }
    }
    pub fn with_deadband(mut self, db: f64) -> Self { self.deadband = db; self }
    
    pub fn update(&mut self, setpoint: f64, measurement: f64) -> i8 {
        let error = setpoint - measurement;
        if error.abs() < self.deadband { return 0; }
        
        self.integral = (self.integral + error).clamp(-100.0, 100.0);
        let deriv = if self.initialized { error - self.prev_error } else { 0.0 };
        self.prev_error = error;
        self.initialized = true;
        
        let output = self.kp * error + self.ki * self.integral + self.kd * deriv;
        if output > 0.0 { 1 } else if output < 0.0 { -1 } else { 0 }
    }
}

// ===== Signal Processing =====

pub fn moving_average(samples: &[i8], window: usize) -> Vec<f64> {
    let n = samples.len();
    let mut result = Vec::with_capacity(n);
    let mut sum = 0i64;
    for i in 0..n {
        sum += samples[i] as i64;
        if i >= window { sum -= samples[i - window] as i64; }
        let count = i.min(window - 1) + 1;
        result.push(sum as f64 / count as f64);
    }
    result
}

pub fn zero_crossing_rate(samples: &[i8]) -> f64 {
    if samples.len() < 2 { return 0.0; }
    let crossings = samples.windows(2).filter(|w| w[0].signum() != w[1].signum()).count();
    crossings as f64 / (samples.len() - 1) as f64
}

// ===== Simple RNG =====

pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self { Self { state: seed } }
    pub fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state as f64 / u64::MAX as f64
    }
    pub fn next_i8(&mut self) -> i8 {
        let r = self.next_f64();
        if r < 0.333 { -1 } else if r < 0.666 { 0 } else { 1 }
    }
}

// ===== Game of Life =====

pub fn step_game_of_life(grid: &TernaryGrid) -> TernaryGrid {
    let mut next = grid.clone();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            let neighbors = grid.neighbors4(x, y);
            let count_pos = neighbors.iter().filter(|&&(nx, ny)| grid.get(nx, ny) == 1).count();
            let count_neg = neighbors.iter().filter(|&&(nx, ny)| grid.get(nx, ny) == -1).count();
            
            let new_state = match cell {
                1 => {
                    // Young cell: survives with 2-3 positive neighbors, ages otherwise
                    if count_pos >= 2 && count_pos <= 3 { 1 }
                    else { 0 }
                }
                0 => {
                    // Dead cell: born if exactly 3 positive neighbors, infected if 3+ negative
                    if count_pos == 3 { 1 }
                    else if count_neg >= 3 { -1 }
                    else { 0 }
                }
                -1 => {
                    // Old cell: dies, can infect neighbors
                    if count_neg >= 2 { -1 } // persists if surrounded by old
                    else { 0 }
                }
                _ => 0,
            };
            next.set(x, y, new_state);
        }
    }
    next
}
