# Hacking on sttt-engine

## Setup

```bash
rustup default stable    # Rust 2024 edition requires recent stable
cargo check              # verify setup
```

## Project Structure

```
src/
├── lib.rs           # public API
└── (modules TBD)    # game types, AI logic
```

## Running Tests

```bash
cargo test           # unit tests
cargo bench          # benchmarks (criterion)
```

## Design Decisions

See [docs/dev/adr/](docs/dev/adr/) for architecture decision records.
