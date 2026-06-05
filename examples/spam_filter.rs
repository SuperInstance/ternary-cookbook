//! ## Demo 2: Ternary Spam Filter
//!
//! Classifies emails as spam (-1), unsure (0), or ham (+1) using
//! simple keyword scoring with ternary thresholds and deadbands.
//!
//! ```bash
//! cargo run --example spam_filter
//! ```

use ternary_cookbook::Ternary;

fn classify_email(subject: &str, body: &str, threshold: f64) -> Ternary {
    // Simple keyword-based scoring
    let spam_words = ["viagra", "lottery", "winner", "free money", "click here", "act now", "congratulations", "urgent", "nigerian", "prince"];
    let ham_words = ["meeting", "project", "report", "deadline", "review", "schedule", "team", "update", "feedback", "agenda"];
    
    let text = format!("{} {}", subject.to_lowercase(), body.to_lowercase());
    
    let mut score = 0.0;
    for word in &spam_words {
        if text.contains(word) { score -= 1.0; }
    }
    for word in &ham_words {
        if text.contains(word) { score += 1.0; }
    }
    
    // Normalize by word count
    let word_count = text.split_whitespace().count().max(1);
    let normalized = score / word_count as f64 * 10.0; // scale up
    
    if normalized > threshold { Ternary::Plus }
    else if normalized < -threshold { Ternary::Minus }
    else { Ternary::Zero }
}

fn main() {
    println!("=== Ternary Spam Filter ===\n");
    
    let emails = [
        ("Team Meeting Tomorrow", "Hi team, let's discuss the project review at 3pm. Please bring your feedback."),
        ("YOU WON THE LOTTERY!!!", "Click here to claim your free money! Act now! Urgent! Nigerian prince needs your help!"),
        ("Q3 Report Ready", "The quarterly report is attached. Schedule a review meeting when you have time."),
        ("URGENT: Your Account", "Congratulations! You are a winner! Click here to verify your lottery prize."),
        ("Lunch?", "Want to grab lunch and discuss the deadline for the project update?"),
        ("Limited Offer!!!", "Free money! Act now! Click here for your prize! Urgent action required!"),
        ("Project Deadline Update", "Hi, the deadline has been moved to Friday. Please update your schedule and send feedback."),
    ];
    
    let threshold = 1.5;
    
    println!("{:<30} | {:<10} | {}", "Subject", "Verdict", "Explanation");
    println!("{:-<30}-+-{:-<10}-+-{:-<40}", "", "", "");
    
    for (subject, body) in &emails {
        let verdict = classify_email(subject, body, threshold);
        let (icon, explanation) = match verdict {
            Ternary::Plus => ("✅ HAM", "Legitimate email — safe to deliver"),
            Ternary::Zero => ("❓ REVIEW", "Uncertain — send to manual review"),
            Ternary::Minus => ("🚫 SPAM", "Spam detected — block or quarantine"),
        };
        
        let display_subject = if subject.len() > 28 { &subject[..28] } else { subject };
        println!("{:<30} | {:<10} | {}", display_subject, icon, explanation);
    }
    
    println!("\n=== Key Takeaway ===");
    println!("Most spam filters are binary (spam/not-spam). Ternary adds a REVIEW state.");
    println!("This is critical: false positives (ham → spam) are expensive in business email.");
    println!("The 0 state catches edge cases that would otherwise cause costly mistakes.");
    println!("\nWith threshold = {:.1}, we control the deadband — wider = fewer false positives.", threshold);
}
