---
why:
    - ../020-goals.kb/performance.md
---

# No Allocation in Hot Path

Move generation and application must not allocate.

## Verification

- `legal_moves` returns iterator or mask, not `Vec`
- `apply` mutates in place or returns cheap copy
- No heap allocations per move in tight loops

This enables millions of moves per second for AI search.
