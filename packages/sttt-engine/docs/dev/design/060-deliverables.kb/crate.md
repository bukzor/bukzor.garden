---
why:
    - ../050-components.kb/game.md
    - ../050-components.kb/mark.md
    - ../050-components.kb/move.md
---

# sttt-engine Crate

The primary deliverable: a Rust library crate.

## Cargo.toml

```toml
[package]
name = "sttt-engine"
version = "0.1.0"
edition = "2024"

[dependencies]
# (minimal - pure logic)

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

## Public API

Re-export from `src/lib.rs`:
- `Mark`
- `Move`
- `BoardStatus`
- `Game`
- `Undo`

## Build

```bash
cargo build           # debug
cargo build --release # optimized
cargo test            # run tests
cargo bench           # run benchmarks
```

## Consumers

This crate has no WASM or platform dependencies. Consumers (e.g., web apps) add their own bindings.
