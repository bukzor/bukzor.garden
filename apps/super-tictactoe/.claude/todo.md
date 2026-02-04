<anthropic-skill-ownership llm-subtask />

- [x] WASM hello-world
- [x] Validate V0 tasks against `docs/private.bukzor-llc/strategy.kb/`
- [ ] V0 milestones
  - [x] Basic tic-tac-toe (3x3)
    - [x] Board rendering
    - [x] Input handling — place X/O via click/touch
    - [x] Win detection
  - [x] Super-ify (3x3 → 9x9)
    - [x] Refactor: generic `check_winner` function
    - [x] 9x9 grid without constraints (data + render + click)
    - [x] Sub-board win indicator
    - [x] Active sub-board constraint
    - [x] Meta-board win detection
  - [x] [Refactor object model: Board/Game/Ui/App separation](todo.kb/2026-02-04-000-refactor-object-model-board-game-ui-app.md)
  - [ ] AI opponent — minimax
    - [ ] Auto-play checkboxes (test harness)
    - abandoned WIP: `archive/auto-play-wip-2026-02-04`
  - [ ] Theme selector — one vanity SKU (e.g. X/O colors)
  - [ ] Support button — "$0.50/mo" external checkout
  - [ ] Deploy — Cloudflare Pages, public URL

## Later

- [ ] Unit tests
- [ ] Undo support
- [ ] Revisit board styling (beyond classic lines-on-white)
