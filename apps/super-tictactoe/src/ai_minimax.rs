//! Best-first AI for Super Tic-Tac-Toe
//!
//! Provides position evaluation and move selection via best-first search with bounded budget.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::bounded_queue::BoundedQueue;
use crate::game::{
    check_winner, count_corners, count_threats, Game, Grid, Mark, Move, Outcome, SubBoard,
};


// =============================================================================
// Score: 6-component lexicographic ordering
// =============================================================================

/// Score with 6 components, evaluated at meta and cell levels.
/// Lexicographic ordering: outcome > threats > corners, meta > cell within each.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, serde::Serialize)]
pub struct Score {
    pub outcome_meta: i32,
    pub outcome_cell: i32,
    pub threats_meta: i32,
    pub threats_cell: i32,
    pub corners_meta: i32,
    pub corners_cell: i32,
}

impl std::ops::Neg for Score {
    type Output = Self;
    fn neg(self) -> Self {
        Score {
            outcome_meta: -self.outcome_meta,
            outcome_cell: -self.outcome_cell,
            threats_meta: -self.threats_meta,
            threats_cell: -self.threats_cell,
            corners_meta: -self.corners_meta,
            corners_cell: -self.corners_cell,
        }
    }
}

// =============================================================================
// SearchState: node in the search tree
// =============================================================================

/// A state in the search tree. Used as key in BoundedQueue.
#[derive(Clone, PartialEq, Eq, Hash)]
struct SearchState {
    game: Game,
    moves: Vec<Move>,    // sequence of moves from root
    scores: Vec<Score>,  // score after each move (negated at opponent turns)
}

// =============================================================================
// Metrics: traits for evaluation predicates
// =============================================================================

/// Grid elements must be copyable and comparable.
trait GridCell: Copy + PartialEq {}
impl<T: Copy + PartialEq> GridCell for T {}

/// A metric that can be evaluated on a grid for a given mark.
trait Metric {
    fn count<T: GridCell>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>, mark: Mark) -> i32;
}

struct MetricOutcome;
struct MetricThreats;
struct MetricCorners;

impl Metric for MetricOutcome {
    fn count<T: GridCell>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>, mark: Mark) -> i32 {
        if check_winner(grid, &to_mark) == Some(mark) { 1 } else { 0 }
    }
}

impl Metric for MetricThreats {
    fn count<T: GridCell>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>, mark: Mark) -> i32 {
        count_threats(grid, to_mark)[mark] as i32
    }
}

impl Metric for MetricCorners {
    fn count<T: GridCell>(grid: &Grid<T>, to_mark: impl Fn(T) -> Option<Mark>, mark: Mark) -> i32 {
        count_corners(grid, to_mark)[mark] as i32
    }
}

// =============================================================================
// Combinators: differential and minimax
// =============================================================================

/// Compute metric(mark) - metric(-mark) for a single grid.
fn differential<M: Metric, T: GridCell>(
    grid: &Grid<T>,
    to_mark: impl Fn(T) -> Option<Mark>,
    mark: Mark,
) -> i32 {
    M::count(grid, &to_mark, mark) - M::count(grid, &to_mark, -mark)
}

/// Compute weighted score: n*max + sum of differentials across sub-boards.
/// Max dominates, sum acts as tiebreaker.
fn minimax<M: Metric>(subs: &Grid<SubBoard>, mark: Mark) -> i32 {
    let to_mark = |m: Mark| (m != Mark::Empty).then_some(m);

    let mut n = 0i32;
    let mut my_max = i32::MIN;
    let mut their_max = i32::MIN;
    let mut my_sum = 0i32;

    for sb in subs.iter().flatten() {
        let d = differential::<M, _>(&sb.cells, to_mark, mark);
        n += 1;
        my_max = my_max.max(d);
        their_max = their_max.max(-d);
        my_sum += d;
    }

    if n == 0 {
        return 0;
    }

    (n * my_max + my_sum) - (n * their_max - my_sum)
}

// =============================================================================
// Evaluate
// =============================================================================

/// Evaluate a position from the perspective of `mark`.
pub fn evaluate(game: &Game, mark: Mark) -> Score {
    let meta = &game.board.sub_boards;
    let to_mark = |sb: SubBoard| match sb.outcome() {
        Outcome::Win(m) => Some(m),
        _ => None,
    };

    Score {
        outcome_meta: differential::<MetricOutcome, _>(meta, &to_mark, mark),
        outcome_cell: minimax::<MetricOutcome>(meta, mark),
        threats_meta: differential::<MetricThreats, _>(meta, &to_mark, mark),
        threats_cell: minimax::<MetricThreats>(meta, mark),
        corners_meta: differential::<MetricCorners, _>(meta, &to_mark, mark),
        corners_cell: minimax::<MetricCorners>(meta, mark),
    }
}

// =============================================================================
// Agent: best-first AI player
// =============================================================================

/// AI agent that picks moves via beam search.
pub struct Agent<D: FnMut(Value) = fn(Value)> {
    /// Maximum search depth.
    pub max_levels: usize,
    /// Maximum states per level.
    pub beam_width: usize,
    /// Time limit for search.
    pub time_limit: Duration,
    debug: D,
}

impl Agent {
    /// Create an agent with default settings and no-op debug.
    pub fn new() -> Self {
        Self {
            max_levels: 1000,
            beam_width: 10_000,
            time_limit: Duration::from_millis(1000),
            debug: |_| {},
        }
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: FnMut(Value)> Agent<D> {
    /// Create an agent with a debug callback.
    pub fn with_debug(debug: D) -> Self {
        Self {
            max_levels: 1000,
            beam_width: 10_000,
            time_limit: Duration::from_millis(1000),
            debug,
        }
    }

    /// Set time limit (builder pattern).
    pub fn time_limit(mut self, limit: Duration) -> Self {
        self.time_limit = limit;
        self
    }

    /// Pick the best moves for the current player using beam search.
    ///
    /// Returns all moves tied for the best score. Caller decides tie-breaking.
    pub fn pick_best(&mut self, game: &Game) -> HashSet<Move> {
        let legal: Vec<_> = game.legal_moves();
        if legal.is_empty() {
            return HashSet::new();
        }

        let start = Instant::now();
        let mover = game.current_turn;

        // Per-level queues for beam search (priority = score path, lexicographic)
        let mut levels: Vec<BoundedQueue<SearchState, Vec<Score>>> = Vec::new();
        levels.push(BoundedQueue::new(self.beam_width));

        // Seed level 0 with states after each first move
        for &mov in &legal {
            let after = game.after(mov);
            // Score from mover's perspective (negate since it's now opponent's turn)
            let score = -evaluate(&after, -mover);
            let scores = vec![score];
            (self.debug)(json!({
                "event": "seed",
                "moves": [mov],
                "scores": scores,
            }));
            levels[0].push(
                SearchState { game: after, moves: vec![mov], scores: scores.clone() },
                scores,
            );
        }

        // Track best path for each first move (lexicographic comparison)
        let mut best_by_first: std::collections::HashMap<Move, Vec<Score>> =
            std::collections::HashMap::new();

        // Round-robin through levels until time exhausted or all empty
        let mut expanded = 0;
        let mut current_level = 0;
        let mut empty_rounds = 0;

        while start.elapsed() < self.time_limit && empty_rounds < levels.len() {
            // Try to pop from current level
            if let Some((state, _)) = levels[current_level].pop() {
                empty_rounds = 0;

                (self.debug)(json!({
                    "event": "expand",
                    "i": expanded,
                    "level": current_level,
                    "moves": state.moves,
                    "scores": state.scores,
                    "outcome": state.game.outcome(),
                }));

                expanded += 1;

                // Track best path for this first move
                let first = state.moves[0];
                let entry = best_by_first.entry(first).or_insert_with(|| state.scores.clone());
                if state.scores > *entry {
                    *entry = state.scores.clone();
                }

                // Terminal state - don't expand further
                if state.game.outcome() != Outcome::InProgress {
                    current_level = (current_level + 1) % levels.len().max(1);
                    continue;
                }

                // Expand children to next level
                let depth = state.moves.len();
                if depth < self.max_levels {
                    // Ensure next level exists
                    while levels.len() <= depth {
                        levels.push(BoundedQueue::new(self.beam_width));
                    }

                    let current_player = state.game.current_turn;
                    for mov in state.game.legal_moves() {
                        let after = state.game.after(mov);
                        // Score from current_player's perspective (negate for next player)
                        let child_score = -evaluate(&after, -current_player);

                        let mut child_moves = state.moves.clone();
                        child_moves.push(mov);
                        let mut child_scores = state.scores.clone();
                        child_scores.push(child_score);

                        levels[depth].push(
                            SearchState {
                                game: after,
                                moves: child_moves,
                                scores: child_scores.clone(),
                            },
                            child_scores,
                        );
                    }
                }
            } else {
                empty_rounds += 1;
            }

            // Move to next level (round-robin)
            current_level = (current_level + 1) % levels.len().max(1);
        }

        // Find best first moves
        if best_by_first.is_empty() {
            return legal.into_iter().collect();
        }

        let best_path = best_by_first.values().max().unwrap().clone();
        (self.debug)(json!({
            "event": "done",
            "best_path": best_path,
            "expanded": expanded,
            "levels": levels.len(),
            "elapsed_ms": start.elapsed().as_millis(),
        }));

        best_by_first
            .into_iter()
            .filter(|(_, path)| *path == best_path)
            .map(|(mov, _)| mov)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::BoardPos;
    use crate::game_builder::GameBuilder;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::Mutex;

    /// Test helper: pick_best with debug output to JSONL file (one per test).
    fn pick_best(game: &Game) -> HashSet<Move> {
        let test_name = std::thread::current()
            .name()
            .unwrap_or("unknown")
            .replace("::", "-");
        let log_path = format!("target/test-{}.jsonl", test_name);

        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(&log_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = Mutex::new(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&log_path)
                .expect("failed to open debug log"),
        );
        Agent::with_debug(|v| {
            let mut f = file.lock().unwrap();
            writeln!(f, "{}", v).unwrap();
        })
        .time_limit(Duration::from_millis(100))
        .pick_best(game)
    }

    /// Test helper: pick_best without debug (for performance tests).
    fn pick_best_quiet(game: &Game) -> HashSet<Move> {
        Agent::new()
            .time_limit(Duration::from_millis(100))
            .pick_best(game)
    }

    /// Test helper: pick_best with longer timeout for deep search tests.
    fn pick_best_deep(game: &Game) -> HashSet<Move> {
        let test_name = std::thread::current()
            .name()
            .unwrap_or("unknown")
            .replace("::", "-");
        let log_path = format!("target/test-{}.jsonl", test_name);
        if let Some(parent) = std::path::Path::new(&log_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = Mutex::new(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&log_path)
                .expect("failed to open debug log"),
        );
        Agent::with_debug(|v| {
            let mut f = file.lock().unwrap();
            writeln!(f, "{}", v).unwrap();
        })
        .time_limit(Duration::from_millis(1000))
        .pick_best(game)
    }

    /// Test helper: count expansions without debug.
    fn count_expansions(game: &Game, time_ms: u64) -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let count = AtomicUsize::new(0);
        Agent::with_debug(|_| { count.fetch_add(1, Ordering::Relaxed); })
            .time_limit(Duration::from_millis(time_ms))
            .pick_best(game);
        count.load(Ordering::Relaxed)
    }

    // =========================================================================
    // Evaluation: Terminal States
    // =========================================================================

    #[test]
    fn evaluate_x_win_returns_positive_for_x() {
        // X wins meta-game via top row (boards (0,0), (0,1), (0,2))
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .subboard_won_by((0, 2), Mark::X)
            .build();

        assert_eq!(evaluate(&game, Mark::X).outcome_meta, 1);
    }

    #[test]
    fn evaluate_x_win_returns_negative_for_o() {
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .subboard_won_by((0, 2), Mark::X)
            .build();

        assert_eq!(evaluate(&game, Mark::O).outcome_meta, -1);
    }

    #[test]
    fn evaluate_o_win_returns_positive_for_o() {
        // O wins via left column (boards (0,0), (1,0), (2,0))
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::O)
            .subboard_won_by((1, 0), Mark::O)
            .subboard_won_by((2, 0), Mark::O)
            .build();

        assert_eq!(evaluate(&game, Mark::O).outcome_meta, 1);
    }

    #[test]
    fn evaluate_draw_returns_zero_outcome() {
        // All 9 sub-boards resolved, no meta-winner
        // Pattern: X wins corners, O wins edges, center drawn
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::O)
            .subboard_won_by((0, 2), Mark::X)
            .subboard_won_by((1, 0), Mark::O)
            .subboard_drawn((1, 1))
            .subboard_won_by((1, 2), Mark::O)
            .subboard_won_by((2, 0), Mark::X)
            .subboard_won_by((2, 1), Mark::O)
            .subboard_won_by((2, 2), Mark::X)
            .build();

        assert_eq!(evaluate(&game, Mark::X).outcome_meta, 0);
        assert_eq!(evaluate(&game, Mark::O).outcome_meta, 0);
    }

    // =========================================================================
    // Evaluation: Heuristics for Non-Terminal Positions
    // =========================================================================

    #[test]
    fn evaluate_prefers_more_subboards_won() {
        let one_board = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .turn(Mark::X)
            .build();

        let two_boards = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .turn(Mark::X)
            .build();

        assert!(evaluate(&two_boards, Mark::X) > evaluate(&one_board, Mark::X));
    }

    #[test]
    fn evaluate_prefers_corner_subboard() {
        // Heuristic values corners (threat potential) over center
        let corner = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)  // corner
            .turn(Mark::X)
            .build();

        let center = GameBuilder::new()
            .subboard_won_by((1, 1), Mark::X)  // center
            .turn(Mark::X)
            .build();

        assert!(evaluate(&corner, Mark::X) > evaluate(&center, Mark::X));
    }

    #[test]
    fn evaluate_is_symmetric_between_players() {
        // Position: X has boards (0,0),(0,1); O has board (2,2)
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .subboard_won_by((2, 2), Mark::O)
            .turn(Mark::X)
            .build();

        let x_score = evaluate(&game, Mark::X);
        let o_score = evaluate(&game, Mark::O);
        // Scores should be negations of each other
        assert_eq!(x_score, -o_score);
    }

    #[test]
    fn evaluate_non_terminal_has_zero_outcome() {
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((1, 1), Mark::O)
            .turn(Mark::X)
            .build();

        let score = evaluate(&game, Mark::X);
        // No winner yet, so outcome_meta is 0
        assert_eq!(score.outcome_meta, 0);
    }

    // =========================================================================
    // Minimax: Tactical Play
    // =========================================================================

    #[test]
    fn minimax_takes_winning_move() {
        // X has boards (0,0), (0,1). Winning board (0,2) completes top row.
        // X needs to win sub-board (0,2) with one move.
        // Set up: X has cells (0,0),(0,1) in board (0,2), needs cell (0,2) to win.
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .cell((0, 2), (0, 0), Mark::X)
            .cell((0, 2), (0, 1), Mark::X)
            // O has some boards elsewhere
            .subboard_won_by((2, 0), Mark::O)
            .subboard_won_by((2, 1), Mark::O)
            .turn(Mark::X)
            .constraint((0, 2))  // Must play in board (0,2)
            .build();

        let best = pick_best(&game);
        assert_eq!(best, HashSet::from([((0, 2), (0, 2)).into()]));
    }

    #[test]
    fn minimax_blocks_opponent_win() {
        // O has boards (0,0), (0,1). O needs board (0,2) to win.
        // X's turn, must play in board (0,2) to block.
        // O has cells (0,0),(0,1) in board (0,2), X must take cell (0,2) to prevent O's sub-board win.
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::O)
            .subboard_won_by((0, 1), Mark::O)
            .cell((0, 2), (0, 0), Mark::O)
            .cell((0, 2), (0, 1), Mark::O)
            .turn(Mark::X)
            .constraint((0, 2))
            .build();

        let best = pick_best_quiet(&game);
        assert_eq!(best, HashSet::from([((0, 2), (0, 2)).into()]));
    }

    #[test]
    fn minimax_takes_subboard_win() {
        // X has cells (0,0),(0,1) in board (1,0). Taking cell (0,2) wins the sub-board.
        let game = GameBuilder::new()
            .cell((1, 0), (0, 0), Mark::X)
            .cell((1, 0), (0, 1), Mark::X)
            .turn(Mark::X)
            .constraint((1, 0))
            .build();

        let best = pick_best(&game);
        assert_eq!(best, HashSet::from([((1, 0), (0, 2)).into()]));
    }

    #[test]
    fn minimax_prefers_winning_now_over_later() {
        // X can win meta immediately via board (0,2), or set up a different win.
        // Should take the immediate win.
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .cell((0, 2), (0, 0), Mark::X)
            .cell((0, 2), (0, 1), Mark::X)
            // Alternative: X also has diagonal progress
            .subboard_won_by((1, 1), Mark::X)
            .turn(Mark::X)
            .constraint((0, 2))
            .build();

        let best = pick_best(&game);
        assert_eq!(best, HashSet::from([((0, 2), (0, 2)).into()]));
    }

    // =========================================================================
    // Search Finds Deeper Tactics
    // =========================================================================

    #[test]
    fn search_finds_forcing_sequence() {
        // O has meta column 1: (0,1)=O, (2,1)=O, needs (1,1).
        // Board (1,1) has O fork: (0,0), (0,2), (2,0) - threatens row 0 AND col 0.
        // O constrained to (0,2). O plays (1,1) → X to board (1,1).
        // X blocks one threat, O wins the other → wins (1,1) → wins meta.
        //
        // Meta-board:        Board (1,1):
        //   X | O | ?          o | _ | o
        //   O | ? | ?    =>    _ | _ | _
        //   O | O | X          o | _ | x
        let game = GameBuilder::new()
            // Meta outcomes
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::O)
            .subboard_won_by((1, 0), Mark::O)
            .subboard_won_by((2, 0), Mark::O)
            .subboard_won_by((2, 1), Mark::O)
            .subboard_won_by((2, 2), Mark::X)
            // Board (1,1): O threatens row 0, X threatens row 2
            .cell((1, 1), (0, 0), Mark::O)
            .cell((1, 1), (0, 2), Mark::O)
            .cell((1, 1), (2, 0), Mark::X)
            .cell((1, 1), (2, 2), Mark::X)
            // Board (0,2): some cells occupied
            .cell((0, 2), (1, 0), Mark::X)
            .cell((0, 2), (2, 0), Mark::O)
            .cell((0, 2), (2, 1), Mark::X)
            .cell((0, 2), (2, 2), Mark::O)
            // Board (1,2): also in progress
            .cell((1, 2), (0, 0), Mark::O)
            .cell((1, 2), (0, 2), Mark::O)
            .turn(Mark::O)
            .constraint((0, 2))
            .build();

        // O plays (0,2):(0,2) → X to (0,2) → X plays (1,1) → O to (1,1) → O wins
        let best = pick_best_deep(&game);

        assert_eq!(best, HashSet::from([((0, 2), (0, 2)).into()]));
    }

    // =========================================================================
    // Correctness Invariants
    // =========================================================================

    #[test]
    fn pick_best_returns_subset_of_legal_moves() {
        let game = Game::new();
        let best = pick_best(&game);
        let legal: HashSet<_> = game.legal_moves().into_iter().collect();
        assert!(best.is_subset(&legal));
        assert!(!best.is_empty());
    }

    #[test]
    fn pick_best_works_when_constrained_to_one_board() {
        // X played in board (1,1) cell (1,1), so O must play in board (1,1)
        let game = GameBuilder::new()
            .cell((1, 1), (1, 1), Mark::X)
            .turn(Mark::O)
            .constraint((1, 1))
            .build();
        let best = pick_best(&game);
        // All best moves must be in board (1,1)
        assert!(best.iter().all(|m| m.board == BoardPos::from((1, 1))));
        let legal: HashSet<_> = game.legal_moves().into_iter().collect();
        assert!(best.is_subset(&legal));
    }

    #[test]
    fn pick_best_works_when_unconstrained() {
        // Constraint board (1,1) is won, so O can play anywhere
        let game = GameBuilder::new()
            .subboard_won_by((1, 1), Mark::X)
            .turn(Mark::O)
            .unconstrained()  // Board (1,1) is resolved, free choice
            .build();

        let best = pick_best(&game);
        let legal: HashSet<_> = game.legal_moves().into_iter().collect();
        assert!(best.is_subset(&legal));
        // No moves should be in resolved board (1,1)
        assert!(best.iter().all(|m| m.board != BoardPos::from((1, 1))));
    }

    // =========================================================================
    // Performance Sanity Checks
    // =========================================================================

    #[test]
    fn expansion_rate_is_reasonable() {
        // Opening position with constraint - moderate branching
        let game = GameBuilder::new()
            .cell((0, 0), (0, 0), Mark::X)
            .turn(Mark::O)
            .constraint((0, 0))
            .build();

        let count = count_expansions(&game, 100);
        let rate = count as f64 / 0.1; // per second

        eprintln!("Expansion rate: {:.0}/sec ({} in 100ms)", rate, count);

        // Minimum 100/sec (debug mode ~200/sec, release ~4000/sec)
        assert!(rate > 100.0,
            "Expansion rate too slow: {:.0}/sec ({} in 100ms)", rate, count);
    }

    #[test]
    fn pick_best_respects_time_limit() {
        // Opening position: huge search space, won't exhaust
        let game = Game::new();

        let start = Instant::now();
        let _ = pick_best_quiet(&game);
        let elapsed = start.elapsed();

        // Should respect ~100ms time limit (with some overhead)
        assert!(elapsed.as_millis() < 200,
            "pick_best took {:?}, expected < 200ms", elapsed);
    }
}
