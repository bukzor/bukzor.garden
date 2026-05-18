---
managed-by: Skill(llm-subtask)
---

# Beam Search AI Implementation

**Priority:** High
**Complexity:** Medium
**Context:** Replacing depth-limited minimax with budget-bounded beam search

## Problem Statement

Depth-limited minimax with alpha-beta pruning doesn't scale for Super TTT:
- Branching factor up to 81 moves early game
- Need deep search to see tactical threats
- Deep search is too slow

## Current Status

Beam search is implemented and working. Core tactical tests pass (immediate wins, blocking).
Performance is ~4500 expansions/sec in release mode. Heuristics need tuning.

### What's Working

- Per-level `BoundedQueue` with round-robin expansion
- Time-based termination (default 1s, 100ms for tests)
- Full path tracking: `Vec<Move>` and `Vec<Score>` per state
- Lexicographic path comparison for proper ordering
- Early-exit optimization in BoundedQueue (skip inserts worse than min)
- Immediate win/block detection passes tests

### What's Not Working

- `search_finds_forcing_sequence` test expects center (1,1) but corner is actually better
  - Corner-first found 21-move win
  - Center-first found 29-move win
  - Test expectation was wrong

### Performance Bottlenecks Identified

1. **Cloning**: Game, Vec<Move>, Vec<Score> cloned for each child (~270μs/expansion)
2. **Path comparison**: Vec<Score> comparison is O(depth)
3. **Hash computation**: SearchState contains full Game + paths

## Implementation Completed

- [x] Add `priority-queue` crate
- [x] Add `Hash + Eq` derives to Game types
- [x] Create `BoundedQueue` wrapper
    - [x] Basic structure in `src/bounded_queue.rs`
    - [x] Add to `lib.rs` module list
    - [x] Tests (8 passing)
    - [x] Early-exit optimization (skip if priority <= min)
- [x] Implement beam search in `Agent::pick_best`
    - [x] `levels: Vec<BoundedQueue<SearchState, Vec<Score>>>`
    - [x] `max_levels: usize` (default 1000)
    - [x] `beam_width: usize` (default 10,000)
    - [x] `time_limit: Duration` (default 1s)
- [x] Round-robin level selection with empty-level skipping
- [x] Track best path per first-move via lexicographic comparison
- [x] Score negation at turn switch for adversarial reasoning

## Remaining Work

- [ ] Fix heuristics (currently testing with corners disabled)
- [ ] Hash Game only (not full path) for deduplication via `push` replace
- [ ] Consider `im::Vector` for O(log n) path cloning
- [ ] Wire into UI (replace random AI)
- [ ] Update test expectations to match correct behavior

## Heuristic Findings

Tested three configurations for positional heuristic:
| Heuristic | First Move | Path Length | Result |
|-----------|------------|-------------|--------|
| Corners   | (2,0)      | 21          | WIN    |
| Centers   | (1,1)      | 29          | WIN    |
| None      | (0,2)      | 27          | no win |

Corner heuristic produces best results. The `n*max + sum` formula (max dominates, sum breaks ties) differentiates positions well.

## Open Questions

- Should we use persistent data structures for paths?
- Can we exploit `DoublePriorityQueue.push()` replacing same-key items for deduplication?
- Nested priority queues for implicit lexicographic ordering?

## Architecture Notes

Key insight from session: full score paths enable proper adversarial comparison without explicit minimax. Lexicographic comparison on `[score₀, score₁, ...]` with negation at opponent turns naturally produces minimax-like ordering.

Design principle: "If you're threading it, you should be holding it" - config bundled into Agent struct with builder pattern for time_limit.
