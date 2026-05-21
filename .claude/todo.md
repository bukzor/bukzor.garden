---
managed-by: Skill(llm-subtask)
cost-benefit-sweh:
  timebox:
    "@value": 2.0
    rationale: |
      Parent index. Child items (Super TTT, Cloudflare Pages, Trunk-
      to-Buck2) rated separately. Residual inline: revisit app file
      layout (~0.5h), create public bukzor-llc repo (~1.5h).
    confidence: tentative
  benefit-2w:
    "@value": 0.3
    rationale: |
      Cleanup items. Modest forward value.
    confidence: tentative
  cost-of-delay-2w:
    "@value": 0.0
    rationale: |
      Layout revisit is cosmetic. Public bukzor-llc repo is a
      strategic move with no per-2w bleed.
    confidence: tentative
---

- [x] WASM hello-world for super-tictactoe
- [ ] Super Tic-Tac-Toe V0 — see `apps/super-tictactoe/.claude/todo.md`
- [ ] [Deploy to Cloudflare Pages](todo.kb/2026-02-05-000-deploy-cloudflare-pages.md) (graduated: script → GHA → Pulumi)

## Later

- [ ] [Trunk to Buck2 migration](todo.kb/2026-02-03-000-trunk-to-buck2-migration.md)
  - Start after TTT ships; evaluate during Worm build
- [ ] Revisit app file layout (index.html at crate root feels odd)
- [ ] Create public bukzor-llc repo (currently simulated at `private.bukzor-llc/public/`)
