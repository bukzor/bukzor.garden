---
depends:
    - skills/llm-collab
---

# Super Tic-Tac-Toe - Development Guide

## Strategic Context

This project is part of bukzor-llc's "three games" calibration experiment.
See `docs/bukzor-llc/` (symlink) for:
- `constitution.kb/` — principles and identity
- `strategy.kb/products.md` — this product's place in the sequence
- `strategy.kb/v0-definition.md` — what "shipped" means

## Quick Reference

**Current state:** Hello-world WASM rendering. No game logic yet.

**V0 requirements:**
- 9x9 board (3x3 meta-grid of 3x3 sub-boards)
- Click to place X/O
- Win detection (sub-board and meta-board)
- AI opponent (minimax)
- "$0.50/mo support" button
- Deploy to Cloudflare Pages

## Architecture

Rust → WASM via Trunk. Single-page app, no backend.

**Key files:**
- `src/lib.rs` — WASM entry point, game logic will live here
- `index.html` — Trunk entry point
- `dist/` — built output (gitignored)

## Build & Run

```bash
trunk serve    # dev server with hot reload
trunk build    # production build to dist/
```

## Conventions

- Keep it simple. This is a calibration project.
- No abstractions until they're earned (see constitution).
- Ship fast, iterate later.
