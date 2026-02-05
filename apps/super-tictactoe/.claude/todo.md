<anthropic-skill-ownership llm-subtask />

- [ ] V0 milestones
  - [ ] Deploy — Cloudflare Pages, public URL
  - [ ] Theme selector — one vanity SKU (e.g. X/O colors)
  - [ ] AI opponent — minimax
    - [x] Auto-play checkboxes (test harness)
    - [ ] Smarter AI — minimax or similar
      - [ ] Board evaluation heuristic — score non-terminal positions
      - [ ] Minimax with alpha-beta pruning + depth limit
      - [ ] Integration — replace pick_random in auto-play
    - [ ] Difficulty slider — smooth adjustment between random and optimal play
  - [ ] Support button — "$0.50/mo" external checkout
    - [ ] Research & choose payment platform (Stripe, Ko-fi, GitHub Sponsors, etc.)
    - [ ] Set up account + configure $0.50/mo product
    - [ ] Add button to page

## Later

- [ ] Sticky inputs — persist auto-play/delay settings via localStorage
- [ ] Persist game state across refresh (localStorage)
- [ ] Undo button
- [ ] Restart button
- [ ] How-to-play hint — collapsible rules explanation for new players
- [ ] Win celebration — visual payoff on meta-game win
- [ ] Score tracking — session win counter across games
- [ ] Revisit turn-indicator UI — consider integrating auto-play controls
- [ ] Tablet layout — improve UI at tablet size, portrait and landscape
- [ ] Fix vertical gap glitch — 1-3px gap between sub-board cell borders
- [ ] Unit tests
- [ ] Revisit board styling (beyond classic lines-on-white)
