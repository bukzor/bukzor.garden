---
depends:
    - skills/llm-collab
---

# Super Tic-Tac-Toe

## V0 Requirements

- 9x9 board (3x3 meta-grid of 3x3 sub-boards)
- Click to place X/O
- Win detection (sub-board and meta-board)
- AI opponent (minimax)
- "$0.50/mo support" button
- Deploy to Cloudflare Pages

## How To

### Build & Run

```bash
trunk serve    # dev server with hot reload
trunk build    # production build to dist/
```

### Deploy

See [`docs/dev/deploy.md`](docs/dev/deploy.md) — manual via wrangler, live at `bukzor-garden--super-tictactoe.pages.dev`

## Architecture

Rust → WASM via Trunk. Single-page app, no backend.

**Key files:**
- `src/lib.rs` — WASM entry point (`App` struct)
- `src/game.rs` — game types and logic (`Mark`, `Board`, `Game`, win detection)
- `src/ai_random.rs` — random move picker (future: `ai_minimax.rs`)
- `src/ui.rs` — DOM rendering and interaction
- `index.html` — Trunk entry point
- `dist/` — built output (gitignored)

## Conventions

**Milestones** in `.claude/todo.md` represent discrete deliverables, each with at least one git commit.
