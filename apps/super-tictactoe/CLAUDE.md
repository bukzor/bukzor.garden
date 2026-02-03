---
depends:
    - skills/llm-collab
---

# Super Tic-Tac-Toe

**Current state:** Hello-world WASM rendering. No game logic yet.

## V0 Requirements

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
