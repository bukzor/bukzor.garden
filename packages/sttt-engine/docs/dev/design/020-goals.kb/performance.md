---
why:
    - ../010-mission.kb/sttt-engine.md
---

# Performance

The engine should be fast enough for AI search algorithms.

AI algorithms (MCTS, minimax) evaluate thousands to millions of positions. The engine must not be the bottleneck.

This goal justifies performance-oriented design choices (bitboards, incremental updates) while not dictating specific implementations.
