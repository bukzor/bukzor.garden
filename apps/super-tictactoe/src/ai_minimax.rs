//! Minimax AI for Super Tic-Tac-Toe
//!
//! Provides position evaluation and move selection via minimax with alpha-beta pruning.

use crate::game::{count_corners, count_threats, Game, Mark, Move, Outcome};

/// Score type: (terminal, threats_diff, corners_diff).
/// Lexicographic ordering: terminal always dominates.
pub type Score = (i32, i32, i32);

/// Win score (terminal component = 10, others zeroed)
pub const SCORE_WIN: Score = (10, 0, 0);

/// Loss score
pub const SCORE_LOSS: Score = (-10, 0, 0);

/// Draw score (slightly worse than continuing)
pub const SCORE_DRAW: Score = (-1, 0, 0);

/// Evaluate a position from the perspective of `player`.
///
/// Returns:
/// - `SCORE_WIN` if player has won the meta-game
/// - `SCORE_LOSS` if opponent has won
/// - `SCORE_DRAW` for draws
/// - Heuristic score for non-terminal positions (based on sub-board control, threats, etc.)
pub fn evaluate(game: &Game, player: Mark) -> Score {
    let opponent = player.next();

    // Check terminal states
    match game.outcome() {
        Outcome::Win(winner) if winner == player => return SCORE_WIN,
        Outcome::Win(_) => return SCORE_LOSS,
        Outcome::Draw => return SCORE_DRAW,
        Outcome::InProgress => {}
    }

    // Heuristic evaluation for non-terminal positions
    let to_mark = |sb: crate::game::SubBoard| match sb.outcome() {
        Outcome::Win(m) => Some(m),
        _ => None,
    };

    let threats = count_threats(&game.board.sub_boards, to_mark);
    let corners = count_corners(&game.board.sub_boards, to_mark);

    let threats_diff = threats[player] as i32 - threats[opponent] as i32;
    let corners_diff = corners[player] as i32 - corners[opponent] as i32;

    (0, threats_diff, corners_diff)
}

/// Pick the best move for the current player using minimax with alpha-beta pruning.
///
/// Uses a default depth limit appropriate for real-time play.
pub fn pick_best(game: &Game) -> Move {
    todo!()
}

/// Pick the best move with an explicit depth limit.
///
/// `depth = 0` returns any legal move (no lookahead).
/// Higher depths search more plies but take longer.
pub fn pick_best_with_depth(game: &Game, depth: usize) -> Move {
    todo!()
}

/// Blend between random and optimal play based on strength (0.0 = random, 1.0 = optimal).
///
/// Used for difficulty adjustment.
pub fn pick_move_with_strength(game: &Game, strength: f64) -> Move {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::BoardPos;
    use crate::game_builder::GameBuilder;

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

        assert_eq!(evaluate(&game, Mark::X), SCORE_WIN);
    }

    #[test]
    fn evaluate_x_win_returns_negative_for_o() {
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .subboard_won_by((0, 2), Mark::X)
            .build();

        assert_eq!(evaluate(&game, Mark::O), SCORE_LOSS);
    }

    #[test]
    fn evaluate_o_win_returns_positive_for_o() {
        // O wins via left column (boards (0,0), (1,0), (2,0))
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::O)
            .subboard_won_by((1, 0), Mark::O)
            .subboard_won_by((2, 0), Mark::O)
            .build();

        assert_eq!(evaluate(&game, Mark::O), SCORE_WIN);
    }

    #[test]
    fn evaluate_draw_returns_draw_score() {
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

        assert_eq!(evaluate(&game, Mark::X), SCORE_DRAW);
        assert_eq!(evaluate(&game, Mark::O), SCORE_DRAW);
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
        // Negate each component for symmetry check
        assert_eq!(x_score, (-o_score.0, -o_score.1, -o_score.2));
    }

    #[test]
    fn evaluate_heuristic_bounded_by_terminal_scores() {
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((1, 1), Mark::O)
            .turn(Mark::X)
            .build();

        let score = evaluate(&game, Mark::X);
        assert!(score > SCORE_LOSS && score < SCORE_WIN);
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

        let chosen = pick_best(&game);
        assert_eq!(chosen, ((0, 2), (0, 2)).into());  // Win board (0,2) → win meta
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

        let chosen = pick_best(&game);
        assert_eq!(chosen, ((0, 2), (0, 2)).into());  // Block O's winning move
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

        let chosen = pick_best(&game);
        assert_eq!(chosen, ((1, 0), (0, 2)).into());  // Complete the row
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

        let chosen = pick_best(&game);
        assert_eq!(chosen, ((0, 2), (0, 2)).into());  // Take the win now
    }

    // =========================================================================
    // Minimax: Depth Limit
    // =========================================================================

    #[test]
    fn depth_zero_returns_legal_move() {
        let game = Game::new();
        let chosen = pick_best_with_depth(&game, 0);
        assert!(game.legal_moves().contains(&chosen));
    }

    #[test]
    fn higher_depth_finds_deeper_tactics() {
        // Position where X can force a sub-board win in 2 moves, but depth 1 doesn't see it.
        // X has cell (0,0) in board (0,0). Playing cell (1,1) (center) creates two threats.
        // O must block one, then X wins with the other.
        //
        // But at depth 1, X just sees "all moves lead to non-terminal" and picks arbitrarily.
        // At depth 2+, X sees that cell (1,1) leads to a guaranteed sub-board win.
        let game = GameBuilder::new()
            .cell((0, 0), (0, 0), Mark::X)
            .turn(Mark::X)
            .constraint((0, 0))
            .build();

        // At depth 1, might pick anything
        let _shallow = pick_best_with_depth(&game, 1);

        // At depth 3+, should find the forcing sequence (center creates fork)
        let deep = pick_best_with_depth(&game, 3);

        // The optimal move is cell (1,1) (center) - creates threats on diag and anti-diag
        assert_eq!(deep, ((0, 0), (1, 1)).into());
    }

    // =========================================================================
    // Minimax: Correctness Invariants
    // =========================================================================

    #[test]
    fn pick_best_always_returns_legal_move() {
        let game = Game::new();
        let chosen = pick_best(&game);
        assert!(game.legal_moves().contains(&chosen));
    }

    #[test]
    fn pick_best_works_when_constrained_to_one_board() {
        // X played in board (1,1) cell (1,1), so O must play in board (1,1)
        let game = GameBuilder::new()
            .cell((1, 1), (1, 1), Mark::X)
            .turn(Mark::O)
            .constraint((1, 1))
            .build();
        let chosen = pick_best(&game);
        assert_eq!(chosen.board, BoardPos::from((1, 1)));  // Must play in board (1,1)
        assert!(game.legal_moves().contains(&chosen));
    }

    #[test]
    fn pick_best_works_when_unconstrained() {
        // Constraint board (1,1) is won, so O can play anywhere
        let game = GameBuilder::new()
            .subboard_won_by((1, 1), Mark::X)
            .turn(Mark::O)
            .unconstrained()  // Board (1,1) is resolved, free choice
            .build();

        let chosen = pick_best(&game);
        assert!(game.legal_moves().contains(&chosen));
        // Can play in any unresolved board (not (1,1))
        assert_ne!(chosen.board, BoardPos::from((1, 1)));
    }

    // =========================================================================
    // Difficulty Adjustment
    // =========================================================================

    #[test]
    fn strength_zero_plays_randomly() {
        let game = Game::new();
        let mut move_counts = std::collections::HashMap::new();

        for _ in 0..100 {
            let chosen = pick_move_with_strength(&game, 0.0);
            *move_counts.entry(chosen).or_insert(0) += 1;
        }

        // Opening has 81 legal moves; should see variety
        assert!(move_counts.len() > 1);
    }

    #[test]
    fn strength_one_plays_optimally() {
        // Same position as minimax_takes_winning_move
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .cell((0, 2), (0, 0), Mark::X)
            .cell((0, 2), (0, 1), Mark::X)
            .subboard_won_by((2, 0), Mark::O)
            .subboard_won_by((2, 1), Mark::O)
            .turn(Mark::X)
            .constraint((0, 2))
            .build();

        let chosen = pick_move_with_strength(&game, 1.0);
        assert_eq!(chosen, ((0, 2), (0, 2)).into());  // Must take the win
    }

    #[test]
    fn strength_half_sometimes_optimal_sometimes_not() {
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::X)
            .cell((0, 2), (0, 0), Mark::X)
            .cell((0, 2), (0, 1), Mark::X)
            .subboard_won_by((2, 0), Mark::O)
            .subboard_won_by((2, 1), Mark::O)
            .turn(Mark::X)
            .constraint((0, 2))
            .build();

        let optimal = ((0, 2), (0, 2)).into();
        let mut optimal_count = 0;

        for _ in 0..100 {
            if pick_move_with_strength(&game, 0.5) == optimal {
                optimal_count += 1;
            }
        }

        // At 0.5 strength, should pick optimal roughly half the time
        // Allow wide margin for randomness
        assert!(optimal_count > 20 && optimal_count < 80,
            "Expected 20-80 optimal moves at strength 0.5, got {}", optimal_count);
    }

    // =========================================================================
    // Performance Sanity Checks
    // =========================================================================

    #[test]
    fn pick_best_completes_in_reasonable_time() {
        use std::time::Instant;

        let game = Game::new();
        let start = Instant::now();
        let _ = pick_best(&game);
        let elapsed = start.elapsed();

        // Should complete within 1 second even from opening
        assert!(elapsed.as_secs() < 1,
            "pick_best took {:?}, expected < 1s", elapsed);
    }

    #[test]
    fn pick_best_fast_in_late_game() {
        use std::time::Instant;

        // Late game: most boards resolved
        let game = GameBuilder::new()
            .subboard_won_by((0, 0), Mark::X)
            .subboard_won_by((0, 1), Mark::O)
            .subboard_won_by((0, 2), Mark::X)
            .subboard_won_by((1, 0), Mark::O)
            .subboard_won_by((1, 2), Mark::X)
            .subboard_won_by((2, 0), Mark::O)
            .subboard_won_by((2, 1), Mark::X)
            .turn(Mark::O)
            .constraint((1, 1))  // Must play in board (1,1)
            .build();

        let start = Instant::now();
        let _ = pick_best(&game);
        let elapsed = start.elapsed();

        // Late game should be very fast
        assert!(elapsed.as_millis() < 100,
            "Late game pick_best took {:?}, expected < 100ms", elapsed);
    }
}
