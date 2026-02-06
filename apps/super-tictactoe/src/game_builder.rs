//! Test utility for constructing game states directly.
//!
//! Allows bypassing normal play rules to set up specific positions for testing.

use crate::game::{BoardPos, CellPos, Game, Mark, Outcome};

/// Builder for constructing test game states.
/// Uses flat indices (0-8) for ergonomics; converts internally.
pub struct GameBuilder {
    game: Game,
}

/// Convert flat index (0-8) to BoardPos.
fn board_pos(idx: usize) -> BoardPos {
    BoardPos { row: idx / 3, col: idx % 3 }
}

/// Convert flat index (0-8) to CellPos.
fn cell_pos(idx: usize) -> CellPos {
    CellPos { row: idx / 3, col: idx % 3 }
}

impl GameBuilder {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    /// Set a cell in a sub-board directly.
    /// `board` and `cell` are flat indices 0-8.
    pub fn cell(mut self, board: usize, cell: usize, mark: Mark) -> Self {
        let bp = board_pos(board);
        let cp = cell_pos(cell);
        self.game.board.sub_boards[bp.row][bp.col].cells[cp.row][cp.col] = mark;
        self
    }

    /// Mark a sub-board as won by a player.
    pub fn subboard_won_by(mut self, board: usize, mark: Mark) -> Self {
        let bp = board_pos(board);
        let sub = &mut self.game.board.sub_boards[bp.row][bp.col];
        // Fill top row with winning marks (outcome computed from cells)
        sub.cells[0][0] = mark;
        sub.cells[0][1] = mark;
        sub.cells[0][2] = mark;
        self
    }

    /// Mark a sub-board as drawn (full, no winner).
    pub fn subboard_drawn(mut self, board: usize) -> Self {
        let bp = board_pos(board);
        let sub = &mut self.game.board.sub_boards[bp.row][bp.col];
        // Fill with a draw pattern (outcome computed from cells):
        // X O X
        // X X O
        // O X O
        sub.cells[0] = [Mark::X, Mark::O, Mark::X];
        sub.cells[1] = [Mark::X, Mark::X, Mark::O];
        sub.cells[2] = [Mark::O, Mark::X, Mark::O];
        self
    }

    /// Set whose turn it is.
    pub fn turn(mut self, mark: Mark) -> Self {
        self.game.current_turn = mark;
        self
    }

    /// Set the constraint (which board must be played in, or None for free choice).
    pub fn constraint(mut self, board: Option<usize>) -> Self {
        self.game.active_sub_board = board.map(board_pos);
        self
    }

    /// Consume the builder and return the constructed game.
    pub fn build(self) -> Game {
        self.game
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // =========================================================================
    // Test Helpers: Flat-index accessors for assertions
    // =========================================================================

    fn get_cell(game: &Game, board: usize, cell: usize) -> Mark {
        let bp = board_pos(board);
        let cp = cell_pos(cell);
        game.sub_board(bp).cells[cp.row][cp.col]
    }

    fn get_subboard_outcome(game: &Game, board: usize) -> Outcome {
        game.sub_board(board_pos(board)).outcome()
    }

    fn subboard_winner(game: &Game, board: usize) -> Option<Mark> {
        match get_subboard_outcome(game, board) {
            Outcome::Win(mark) => Some(mark),
            _ => None,
        }
    }

    fn is_resolved(game: &Game, board: usize) -> bool {
        get_subboard_outcome(game, board) != Outcome::InProgress
    }

    // =========================================================================
    // new(): Creates Empty Game
    // =========================================================================

    #[test]
    fn new_creates_empty_board() {
        let game = GameBuilder::new().build();

        // All 81 cells should be empty
        for board in 0..9 {
            for cell in 0..9 {
                assert_eq!(get_cell(&game, board, cell), Mark::Empty);
            }
        }
    }

    #[test]
    fn new_starts_with_x_turn() {
        let game = GameBuilder::new().build();
        assert_eq!(game.current_turn, Mark::X);
    }

    #[test]
    fn new_starts_unconstrained() {
        let game = GameBuilder::new().build();
        assert_eq!(game.active_sub_board, None);
    }

    #[test]
    fn new_has_no_winner() {
        let game = GameBuilder::new().build();
        assert_eq!(game.winner(), None);
    }

    // =========================================================================
    // cell(): Direct Cell Placement
    // =========================================================================

    #[test]
    fn cell_places_mark() {
        let game = GameBuilder::new()
            .cell(3, 5, Mark::X)
            .build();

        assert_eq!(get_cell(&game, 3, 5), Mark::X);
    }

    #[test]
    fn cell_can_place_multiple() {
        let game = GameBuilder::new()
            .cell(0, 0, Mark::X)
            .cell(0, 1, Mark::O)
            .cell(4, 4, Mark::X)
            .build();

        assert_eq!(get_cell(&game, 0, 0), Mark::X);
        assert_eq!(get_cell(&game, 0, 1), Mark::O);
        assert_eq!(get_cell(&game, 4, 4), Mark::X);
    }

    #[test]
    fn cell_does_not_affect_other_cells() {
        let game = GameBuilder::new()
            .cell(2, 7, Mark::O)
            .build();

        // The set cell has the mark
        assert_eq!(get_cell(&game, 2, 7), Mark::O);

        // Adjacent cells are still empty
        assert_eq!(get_cell(&game, 2, 6), Mark::Empty);
        assert_eq!(get_cell(&game, 2, 8), Mark::Empty);
        assert_eq!(get_cell(&game, 1, 7), Mark::Empty);
    }

    #[test]
    fn cell_can_overwrite() {
        // This is test-only behavior; normal play doesn't allow this
        let game = GameBuilder::new()
            .cell(0, 0, Mark::X)
            .cell(0, 0, Mark::O)
            .build();

        assert_eq!(get_cell(&game, 0, 0), Mark::O);
    }

    // =========================================================================
    // subboard_won_by(): Mark Sub-board as Won
    // =========================================================================

    #[test]
    fn subboard_won_by_marks_winner() {
        let game = GameBuilder::new()
            .subboard_won_by(4, Mark::X)
            .build();

        assert_eq!(subboard_winner(&game, 4), Some(Mark::X));
    }

    #[test]
    fn subboard_won_by_fills_winning_cells() {
        let game = GameBuilder::new()
            .subboard_won_by(0, Mark::O)
            .build();

        // Should have 3 O's in a winning line
        let o_count = (0..9)
            .filter(|&c| get_cell(&game, 0, c) == Mark::O)
            .count();
        assert!(o_count >= 3);
    }

    #[test]
    fn subboard_won_by_removes_from_legal_moves() {
        let game = GameBuilder::new()
            .subboard_won_by(2, Mark::X)
            .constraint(None)
            .build();

        // No legal moves in won board
        let bp = board_pos(2);
        let moves_in_board_2: Vec<_> = game.legal_moves()
            .into_iter()
            .filter(|m| m.board == bp)
            .collect();
        assert!(moves_in_board_2.is_empty());
    }

    #[test]
    fn subboard_won_by_multiple_boards() {
        let game = GameBuilder::new()
            .subboard_won_by(0, Mark::X)
            .subboard_won_by(4, Mark::X)
            .subboard_won_by(8, Mark::O)
            .build();

        assert_eq!(subboard_winner(&game, 0), Some(Mark::X));
        assert_eq!(subboard_winner(&game, 4), Some(Mark::X));
        assert_eq!(subboard_winner(&game, 8), Some(Mark::O));
    }

    #[test]
    fn subboard_won_by_three_in_row_wins_meta() {
        let game = GameBuilder::new()
            .subboard_won_by(0, Mark::X)
            .subboard_won_by(1, Mark::X)
            .subboard_won_by(2, Mark::X)
            .build();

        assert_eq!(game.winner(), Some(Mark::X));
    }

    // =========================================================================
    // subboard_drawn(): Mark Sub-board as Drawn
    // =========================================================================

    #[test]
    fn subboard_drawn_has_no_winner() {
        let game = GameBuilder::new()
            .subboard_drawn(5)
            .build();

        assert_eq!(subboard_winner(&game, 5), None);
    }

    #[test]
    fn subboard_drawn_is_full() {
        let game = GameBuilder::new()
            .subboard_drawn(3)
            .build();

        // All 9 cells should be filled
        for cell in 0..9 {
            assert_ne!(get_cell(&game, 3, cell), Mark::Empty);
        }
    }

    #[test]
    fn subboard_drawn_is_resolved() {
        let game = GameBuilder::new()
            .subboard_drawn(7)
            .build();

        assert!(is_resolved(&game, 7));
    }

    #[test]
    fn subboard_drawn_removes_from_legal_moves() {
        let game = GameBuilder::new()
            .subboard_drawn(1)
            .constraint(None)
            .build();

        let bp = board_pos(1);
        let moves_in_board_1: Vec<_> = game.legal_moves()
            .into_iter()
            .filter(|m| m.board == bp)
            .collect();
        assert!(moves_in_board_1.is_empty());
    }

    // =========================================================================
    // turn(): Set Current Player
    // =========================================================================

    #[test]
    fn turn_sets_x() {
        let game = GameBuilder::new()
            .turn(Mark::X)
            .build();

        assert_eq!(game.current_turn, Mark::X);
    }

    #[test]
    fn turn_sets_o() {
        let game = GameBuilder::new()
            .turn(Mark::O)
            .build();

        assert_eq!(game.current_turn, Mark::O);
    }

    #[test]
    fn turn_overrides_default() {
        // Default is X, but we can override to O
        let game = GameBuilder::new()
            .turn(Mark::O)
            .build();

        assert_eq!(game.current_turn, Mark::O);
    }

    // =========================================================================
    // constraint(): Set Board Constraint
    // =========================================================================

    #[test]
    fn constraint_none_allows_any_board() {
        let game = GameBuilder::new()
            .constraint(None)
            .build();

        assert_eq!(game.active_sub_board, None);

        // Should have moves in multiple boards
        let boards: HashSet<_> = game.legal_moves()
            .into_iter()
            .map(|m| m.board)
            .collect();
        assert!(boards.len() > 1);
    }

    #[test]
    fn constraint_some_restricts_to_board() {
        let game = GameBuilder::new()
            .constraint(Some(6))
            .build();

        let bp = board_pos(6);
        assert_eq!(game.active_sub_board, Some(bp));

        // All legal moves should be in board 6
        for mov in game.legal_moves() {
            assert_eq!(mov.board, bp);
        }
    }

    #[test]
    fn unconstrained_skips_resolved_board() {
        // When unconstrained (e.g., target board was resolved), can play
        // in any unresolved board but not the resolved one.
        let game = GameBuilder::new()
            .subboard_won_by(4, Mark::X)
            .constraint(None)
            .build();

        let bp4 = board_pos(4);
        let boards: HashSet<_> = game.legal_moves()
            .into_iter()
            .map(|m| m.board)
            .collect();
        assert_eq!(boards.len(), 8); // all except board 4
        assert!(!boards.contains(&bp4));
    }

    // =========================================================================
    // Chaining: Methods Compose Correctly
    // =========================================================================

    #[test]
    fn methods_chain_correctly() {
        let game = GameBuilder::new()
            .subboard_won_by(0, Mark::X)
            .subboard_won_by(1, Mark::X)
            .cell(2, 0, Mark::X)
            .cell(2, 1, Mark::X)
            .turn(Mark::X)
            .constraint(Some(2))
            .build();

        assert_eq!(subboard_winner(&game, 0), Some(Mark::X));
        assert_eq!(subboard_winner(&game, 1), Some(Mark::X));
        assert_eq!(get_cell(&game, 2, 0), Mark::X);
        assert_eq!(get_cell(&game, 2, 1), Mark::X);
        assert_eq!(game.current_turn, Mark::X);
        assert_eq!(game.active_sub_board, Some(board_pos(2)));
    }

    #[test]
    fn order_independent_for_non_overlapping_operations() {
        let game1 = GameBuilder::new()
            .cell(0, 0, Mark::X)
            .turn(Mark::O)
            .constraint(Some(3))
            .build();

        let game2 = GameBuilder::new()
            .constraint(Some(3))
            .turn(Mark::O)
            .cell(0, 0, Mark::X)
            .build();

        assert_eq!(get_cell(&game1, 0, 0), get_cell(&game2, 0, 0));
        assert_eq!(game1.current_turn, game2.current_turn);
        assert_eq!(game1.active_sub_board, game2.active_sub_board);
    }
}
