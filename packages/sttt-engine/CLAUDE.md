--- # workaround: anthropics/claude-code#13003
depends:
    - skills/llm-collab
    - skills/llm-subtask
---

# sttt-engine — Development Guide

Rust library for Super Tic-Tac-Toe game logic.

## Current Work

Check `.claude/todo.md` and `.claude/todo.kb/` for active efforts. Load `Skill("llm-subtask")` for maintenance.

## Architecture

Pure Rust library. No WASM dependencies — that's handled by consumers.

**Key modules:**
- `src/lib.rs` — public API (re-exports)
- Game types: `Mark`, `Board`, `Game`, win detection

## Key Files

- `src/lib.rs` — library root
- `Cargo.toml` — Rust 2024 edition, rand + criterion

## Conventions

- Keep library pure — no platform-specific code
- Consumer (`apps/super-tictactoe`) handles WASM integration

## Testing

```bash
cargo test              # unit tests
cargo bench             # criterion benchmarks (in benches/)
```
