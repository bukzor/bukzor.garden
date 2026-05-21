---
managed-by: Skill(llm-subtask)
cost-benefit-sweh:
  timebox:
    "@value": 4.0
    rationale: |
      ~1h define core types (Mark, Board, Game), ~1.5h implement win
      detection with tests, ~1h move validation logic with tests,
      ~0.5h misc plumbing. Foundation work for super-tictactoe.
    confidence: tentative
  benefit-2w:
    "@value": 1.0
    rationale: |
      Unblocks gameplay — without engine types, no AI, no UI loop, no
      anything in super-tictactoe. Forward value ~$100 in window (the
      foundation that downstream work builds on).
    confidence: tentative
  cost-of-delay-2w:
    "@value": 0.2
    rationale: |
      Super-tictactoe is part of the v0-blocker chain. Each 2w of
      delay pushes shipping out by 2w. Hobby-scale project, so
      modest cod; non-zero because it's the gating dependency.
    confidence: tentative
---

- [ ] Define core types (Mark, Board, Game)
- [ ] Implement win detection
- [ ] Add move validation (constraint logic)

## Later

- [ ] Benchmarks
