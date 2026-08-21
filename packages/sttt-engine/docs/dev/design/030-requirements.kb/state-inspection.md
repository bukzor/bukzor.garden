---
why:
    - ../020-goals.kb/generic-api.md
    - ../020-goals.kb/testability.md
---

# State Inspection

Consumers must be able to inspect game state.

## Verification

- Can query which player occupies any cell
- Can query current player to move
- Can query sub-board status (open/won-by-X/won-by-O/drawn)
- Can query meta-board status (ongoing/won-by-X/won-by-O/drawn)
- Can query which board(s) are valid targets for next move
