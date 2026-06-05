# Ternary in Practice: The Spam Filter

*A case study in ternary classification — why a third state beats binary filters.*

---

## The Problem

Email spam filters have an eternal problem: what to do with emails that are *probably* spam but not definitely.

A binary filter (spam/ham) forces a choice:
- **Threshold too high**: Spam leaks through.
- **Threshold too low**: Legitimate email gets flagged.

Every spam filter builder has tried to solve this with scoring systems, multiple rules, Bayesian fusion — all trying to approximate a middle ground that binary can't express.

## The Ternary Approach

```rust
use ternary_types::Ternary;

/// Three outcomes for any incoming email
#[derive(Debug)]
enum EmailVerdict {
    Spam  (Ternary::Negative),  // Definitely spam
    Review(Ternary::Neutral),   // Uncertain — needs human check
    Ham   (Ternary::Positive),  // Definitely clean
}
```

The $0$ (neutral) state is the safety valve. When rules disagree, signals conflict, or the model is below confidence threshold, the email goes to **Review** instead of being forced into spam or ham.

## The Deadband Effect

Binary filters oscillate — small changes in content push emails past the threshold, flipping them from ham to spam and back. This is "threshold thrashing."

Ternary's neutral zone (the deadband) absorbs these fluctuations:

```
Binary threshold:     [---spam---|--------ham--------]
                      ↑ borderline cases flip here

Ternary zones:  [spam] [review] [ham]
                    ↑    ↑    ↑
            certain  uncertain  certain
```

Emails in the middle (review) don't flip to spam or ham. They stay in review until a human (or stronger signal) pushes them definitively one way.

## The Result

- **No false positives**: Anything uncertain goes to review, not to spam.
- **No false negatives**: Uncertain spam doesn't reach the inbox either.
- **No threshold thrashing**: The deadband absorbs normal signal fluctuation.

People trust a system that says "I don't know" more than one that's confidently wrong 5% of the time.

## Try It

```bash
cargo run --example spam_filter
```

The demo classifies incoming messages using configurable ternary rules and shows the classification boundary vs. a binary equivalent.

## Case Study Takeaway

The $0$ state isn't failure. It's the space where your system admits uncertainty — and that admission builds more trust than any false binary confidence.

*See also: **[From Binary to Ternary](https://github.com/SuperInstance/ternary-cookbook/blob/master/guides/FROM_BINARY.md)**, **[Ternary for the Rest of Us](https://github.com/SuperInstance/ternary-types/docs/TUTORIAL.md)***
