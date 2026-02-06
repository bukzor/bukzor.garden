# Devlog: Minimax AI Design

## Focus

Design session for minimax AI implementation. No code written — capturing
architecture decisions and design heuristics for future sessions.

## Decisions

### Pure functions over stateful classes

Rejected "Evaluator", "Searcher", "Selector" class names. These smell of Java
Enterprise — implying stateful objects when simple functions suffice.

```rust
// Yes
fn evaluate(game: &Game, player: Mark) -> Score
fn search(game: &Game, depth: usize, ...) -> Score

// No
struct Evaluator { ... }
impl Evaluator { fn evaluate(&self, ...) }
```

Use structs only when there's state to manage (transposition tables, configurable
weights). For V0, there isn't.

### Tuple score with lexicographic ordering

Score is `(i32, i32, i32)` — terminal status, threats differential, corners
differential. Rust tuples implement `Ord` lexicographically, so comparison works
automatically.

```rust
type Score = (i32, i32, i32);
const WIN: Score = (10, 0, 0);
const LOSS: Score = (-10, 0, 0);
const DRAW: Score = (-1, 0, 0);  // slightly worse than continuing
// Non-terminal: (0, my_threats - theirs, my_corners - theirs)
```

**Why not weighted sum?** Weights require tuning. With lexicographic ordering,
priorities are explicit: terminal always dominates, then threats, then corners.
No magic numbers to adjust.

**Why DRAW = -1?** Slightly prefer continuing to play over accepting a draw.
Aggressive play style.

### Generic Grid functions

Both sub-boards (3x3 cells) and meta-board (3x3 sub-boards) have the same shape.
Reuse logic via generics:

```rust
type Grid<T> = [[T; 3]; 3];

fn check_winner<T: Copy>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>) -> Option<Mark>
fn count_corners<T: Copy>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>) -> EnumMap<Mark, usize>
fn count_threats<T: Copy>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>) -> EnumMap<Mark, usize>
```

The `to_mark` adapter handles the difference: cells contain `Mark`, sub-boards
contain `SubBoard` (extract winner from outcome).

### EnumMap over hand-rolled ByMark<T>

`enum_map` crate is a community staple (~10M downloads). Use it instead of
writing our own 15-line version. Signals "this is the standard pattern."

### Tie-breaking: random among equals

When multiple moves have the same best score, pick randomly. Don't try to
collapse symmetric positions in the search graph — symmetry in Super TTT is
complex (meta-grid symmetry × constraint state).

For deterministic tests: seed the RNG.

## Design Heuristics (general)

- **Name functions by what they compute, not their category.** `evaluate` not
  `Evaluator`, `search` not `Searcher`.
- **Tuple types for multi-priority ordering.** Lexicographic comparison is free
  and explicit.
- **Generic + adapter pattern for similar-shaped data.** One function, multiple
  use sites.
- **Use established crates for standard patterns.** Don't reinvent EnumMap.

## Implementation Plan

See `.claude/todo.md` for step-by-step commits. Key sequence:

1. Add `enum_map`, derive `Enum` on `Mark`
2. Add `Grid<T>` typedef
3. Add `count_corners`, `count_threats`
4. Add `Score` type and `score()` function
5. Add `search()` with alpha-beta
6. Add `pick_best()` public API
7. Wire into lib.rs, integrate with auto-play

## Open Questions

- Depth limit for real-time play? Need to benchmark.
- Difficulty slider: blend random and optimal by what mechanism?
