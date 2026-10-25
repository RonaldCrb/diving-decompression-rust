# diving-decompression (`v0.1.9`)

Rust library (`#![forbid(unsafe_code)]`) for calculating air diving decompression schedules, no-decompression limits (NDL), repetitive group letters, and residual nitrogen times (RNT) based on the **U.S. Navy Diving Manual (Revision 7)**.

Initially prototyped in TypeScript and ported to Rust to enforce strict compile-time type safety and memory safety guarantees (`#![forbid(unsafe_code)]`, `#![deny(missing_docs)]`) for hyperbaric and commercial diving calculations.

## Features

- **No-Decompression Limits & Group Letters (`Dive`):** Computes NDL (in minutes) and repetitive group designations up to 190 fsw.
- **Decompression Stop Profiles (`Dive::deco_dive`):** Returns full U.S. Navy Rev 7 air and in-water/surface O₂ (`SurDO2`) decompression stop schedules, total ascent times (`TAT`), oxygen central nervous system toxicity periods (`o2cp`), and exceptional exposure flags.
- **Repetitive Dive Planning (`DivePlan`):** Calculates post-surface-interval repetitive group letters and residual nitrogen time (`RNT`) for subsequent dives.

## Usage

```rust
use diving_decompression::{Dive, DivePlan};

fn main() {
    // Single dive: 46 fsw for 60 minutes
    let dive = Dive::new(46, 60);
    let ndl = dive.no_decompression_limit(); // 92 minutes
    let group = dive.group_letter();         // "I"

    // Repetitive dive plan: 60 min surface interval, next dive to 50 fsw
    let plan = DivePlan::from_dive(dive, 60, 50);
    let new_group = plan.repetitive_group_letter();
    let rnt = plan.residual_nitrogen_time();
}
```

## Running Tests

```bash
cargo test
```

## Safety Notice

This crate is provided as an open-source reference implementation of the U.S. Navy Diving Manual Revision 7 tables. Decompression models cannot account for individual physiological variables (water temperature, hydration, workload, or patent foramen ovale). Always verify operational dive plans against certified hyperbaric tables and standards.

## License

GPL-3.0-or-later
