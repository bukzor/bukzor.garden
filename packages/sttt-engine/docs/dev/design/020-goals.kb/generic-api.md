---
why:
    - ../010-mission.kb/sttt-engine.md
---

# Generic API

The engine should expose a clean interface usable by various consumers.

- UI applications can render and accept moves
- AI algorithms can search game trees
- Test harnesses can verify behavior
- Future consumers we haven't imagined

API design should not assume a specific consumer. Game logic is the library's concern; how to use it is the consumer's.
