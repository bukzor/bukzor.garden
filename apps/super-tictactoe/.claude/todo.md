<anthropic-skill-ownership llm-subtask />

- [x] WASM hello-world
- [x] Validate V0 tasks against `docs/private.bukzor-llc/strategy.kb/`
- [ ] V0 milestones
  - [x] Basic tic-tac-toe (3x3)
    - [x] Board rendering
    - [x] Input handling — place X/O via click/touch
    - [x] Win detection
  - [ ] Super-ify (3x3 → 9x9)
    - [x] Refactor: generic `check_winner` function
    - [ ] 9x9 grid without constraints (data + render + click)
    - [ ] Active sub-board constraint
    - [ ] Meta-board win detection
  - [ ] AI opponent — minimax
  - [ ] Theme selector — one vanity SKU (e.g. X/O colors)
  - [ ] Support button — "$0.50/mo" external checkout
  - [ ] Deploy — Cloudflare Pages, public URL

## Later

- [ ] Undo support
- [ ] Revisit board styling (beyond classic lines-on-white)
