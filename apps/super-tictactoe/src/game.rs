#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mark {
    Empty,
    X,
    O,
}

impl Mark {
    pub fn symbol(self) -> &'static str {
        match self {
            Mark::Empty => "",
            Mark::X => "X",
            Mark::O => "O",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::Empty => Mark::X,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outcome {
    InProgress,
    Win(Mark),
    Draw,
}

const fn winning_lines() -> [[(usize, usize); 3]; 8] {
    let mut lines = [[(0, 0); 3]; 8];

    // NOTE: `for` in const fn requires const iterators (not yet stable)
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 3 {
            lines[i][j] = (i, j);      // row i
            lines[3 + i][j] = (j, i);  // col i
            j += 1;
        }
        lines[6][i] = (i, i);          // main diagonal
        lines[7][i] = (i, 2 - i);      // anti-diagonal
        i += 1;
    }
    lines
}

pub const WINNING_LINES: [[(usize, usize); 3]; 8] = winning_lines();

pub fn check_winner<T: Copy + PartialEq>(
    grid: &[[T; 3]; 3],
    to_mark: impl Fn(T) -> Option<Mark>,
) -> Option<Mark> {
    for line in WINNING_LINES {
        let [a, b, c] = line.map(|(r, c)| to_mark(grid[r][c]));
        if a.is_some() && a == b && b == c {
            return a;
        }
    }
    None
}

#[derive(Clone, Copy, PartialEq)]
pub struct SubBoard {
    pub cells: [[Mark; 3]; 3],
    pub outcome: Outcome,
}

impl SubBoard {
    pub fn new() -> Self {
        Self {
            cells: [[Mark::Empty; 3]; 3],
            outcome: Outcome::InProgress,
        }
    }

    pub fn check_winner(&self) -> Option<Mark> {
        check_winner(&self.cells, |m| (m != Mark::Empty).then_some(m))
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|&m| m != Mark::Empty)
    }

    pub fn play(&mut self, row: usize, col: usize, mark: Mark) -> bool {
        if self.outcome != Outcome::InProgress {
            return false;
        }
        if self.cells[row][col] != Mark::Empty {
            return false;
        }
        self.cells[row][col] = mark;
        if let Some(winner) = self.check_winner() {
            self.outcome = Outcome::Win(winner);
        } else if self.is_full() {
            self.outcome = Outcome::Draw;
        }
        true
    }
}

pub struct Board {
    pub sub_boards: [[SubBoard; 3]; 3],
    pub outcome: Outcome,
}

impl Board {
    pub fn new() -> Self {
        Self {
            sub_boards: [[SubBoard::new(); 3]; 3],
            outcome: Outcome::InProgress,
        }
    }
}

pub struct Game {
    pub board: Board,
    pub current_turn: Mark,
    pub active_sub_board: Option<(usize, usize)>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_turn: Mark::X,
            active_sub_board: None,
        }
    }

    pub fn check_winner(&self) -> Option<Mark> {
        check_winner(&self.board.sub_boards, |sb| match sb.outcome {
            Outcome::Win(mark) => Some(mark),
            _ => None,
        })
    }

    pub fn is_full(&self) -> bool {
        self.board.sub_boards
            .iter()
            .flatten()
            .all(|sb| sb.outcome != Outcome::InProgress)
    }

    pub fn outcome(&self) -> Outcome {
        self.board.outcome
    }

    pub fn play(&mut self, meta_row: usize, meta_col: usize, row: usize, col: usize) -> bool {
        if self.board.outcome != Outcome::InProgress {
            return false;
        }
        if let Some((ar, ac)) = self.active_sub_board {
            if (meta_row, meta_col) != (ar, ac) {
                return false;
            }
        }
        let sub_board = &mut self.board.sub_boards[meta_row][meta_col];
        if !sub_board.play(row, col, self.current_turn) {
            return false;
        }
        if let Some(winner) = self.check_winner() {
            self.board.outcome = Outcome::Win(winner);
            self.current_turn = Mark::Empty;
            self.active_sub_board = None;
        } else if self.is_full() {
            self.board.outcome = Outcome::Draw;
            self.current_turn = Mark::Empty;
            self.active_sub_board = None;
        } else {
            self.current_turn = self.current_turn.next();
            let target = &self.board.sub_boards[row][col];
            self.active_sub_board = if target.outcome == Outcome::InProgress {
                Some((row, col))
            } else {
                None
            };
        }
        true
    }

    pub fn legal_moves(&self) -> Vec<(usize, usize, usize, usize)> {
        if self.board.outcome != Outcome::InProgress {
            return Vec::new();
        }
        let mut moves = Vec::new();
        let boards_to_check: Vec<(usize, usize)> = match self.active_sub_board {
            Some(pos) => vec![pos],
            None => (0..3).flat_map(|r| (0..3).map(move |c| (r, c))).collect(),
        };
        for (mr, mc) in boards_to_check {
            let sub = &self.board.sub_boards[mr][mc];
            if sub.outcome != Outcome::InProgress {
                continue;
            }
            for r in 0..3 {
                for c in 0..3 {
                    if sub.cells[r][c] == Mark::Empty {
                        moves.push((mr, mc, r, c));
                    }
                }
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Mark --

    #[test]
    fn mark_next_alternates() {
        assert_eq!(Mark::X.next(), Mark::O);
        assert_eq!(Mark::O.next(), Mark::X);
        assert_eq!(Mark::Empty.next(), Mark::X);
    }

    // -- SubBoard --

    #[test]
    fn sub_board_starts_empty() {
        let sb = SubBoard::new();
        assert_eq!(sb.outcome, Outcome::InProgress);
        for row in &sb.cells {
            for &cell in row {
                assert_eq!(cell, Mark::Empty);
            }
        }
    }

    #[test]
    fn sub_board_play_places_mark() {
        let mut sb = SubBoard::new();
        assert!(sb.play(0, 0, Mark::X));
        assert_eq!(sb.cells[0][0], Mark::X);
    }

    #[test]
    fn sub_board_rejects_occupied_cell() {
        let mut sb = SubBoard::new();
        sb.play(0, 0, Mark::X);
        assert!(!sb.play(0, 0, Mark::O));
        assert_eq!(sb.cells[0][0], Mark::X);
    }

    #[test]
    fn sub_board_rejects_play_after_win() {
        let mut sb = SubBoard::new();
        sb.play(0, 0, Mark::X);
        sb.play(0, 1, Mark::X);
        sb.play(0, 2, Mark::X);
        assert_eq!(sb.outcome, Outcome::Win(Mark::X));
        assert!(!sb.play(1, 0, Mark::O));
    }

    #[test]
    fn sub_board_row_win() {
        let mut sb = SubBoard::new();
        for c in 0..3 {
            sb.play(1, c, Mark::O);
        }
        assert_eq!(sb.outcome, Outcome::Win(Mark::O));
    }

    #[test]
    fn sub_board_col_win() {
        let mut sb = SubBoard::new();
        for r in 0..3 {
            sb.play(r, 2, Mark::X);
        }
        assert_eq!(sb.outcome, Outcome::Win(Mark::X));
    }

    #[test]
    fn sub_board_diagonal_win() {
        let mut sb = SubBoard::new();
        for i in 0..3 {
            sb.play(i, i, Mark::X);
        }
        assert_eq!(sb.outcome, Outcome::Win(Mark::X));
    }

    #[test]
    fn sub_board_anti_diagonal_win() {
        let mut sb = SubBoard::new();
        for i in 0..3 {
            sb.play(i, 2 - i, Mark::O);
        }
        assert_eq!(sb.outcome, Outcome::Win(Mark::O));
    }

    #[test]
    fn sub_board_draw() {
        let mut sb = SubBoard::new();
        // X O X
        // X X O
        // O X O
        let moves = [
            (0, 0, Mark::X), (0, 1, Mark::O), (0, 2, Mark::X),
            (1, 0, Mark::X), (1, 1, Mark::X), (1, 2, Mark::O),
            (2, 0, Mark::O), (2, 1, Mark::X), (2, 2, Mark::O),
        ];
        for (r, c, mark) in moves {
            sb.play(r, c, mark);
        }
        assert_eq!(sb.outcome, Outcome::Draw);
    }

    // -- Game: basics --

    #[test]
    fn game_starts_with_x() {
        let g = Game::new();
        assert_eq!(g.current_turn, Mark::X);
        assert_eq!(g.active_sub_board, None);
        assert_eq!(g.outcome(), Outcome::InProgress);
    }

    #[test]
    fn game_first_move_anywhere() {
        let g = Game::new();
        // all 81 cells should be legal
        assert_eq!(g.legal_moves().len(), 81);
    }

    #[test]
    fn game_alternates_turns() {
        let mut g = Game::new();
        g.play(0, 0, 0, 0);
        assert_eq!(g.current_turn, Mark::O);
        g.play(0, 0, 1, 1);
        assert_eq!(g.current_turn, Mark::X);
    }

    // -- Game: constraint enforcement --

    #[test]
    fn game_constrains_to_target_sub_board() {
        let mut g = Game::new();
        // play in sub-board (0,0), cell (1,2) → sends opponent to sub-board (1,2)
        g.play(0, 0, 1, 2);
        assert_eq!(g.active_sub_board, Some((1, 2)));

        // playing in wrong sub-board is rejected
        assert!(!g.play(0, 0, 0, 1));
        // playing in correct sub-board works
        assert!(g.play(1, 2, 0, 0));
    }

    #[test]
    fn game_frees_constraint_when_target_resolved() {
        let mut g = Game::new();
        g.board.sub_boards[0][0].outcome = Outcome::Win(Mark::X);
        // Play into cell (0,0) of sub-board (1,1) → target is (0,0) which is resolved
        g.play(1, 1, 0, 0);
        assert_eq!(g.active_sub_board, None);
    }

    // -- Game: legal_moves --

    #[test]
    fn legal_moves_respects_constraint() {
        let mut g = Game::new();
        g.play(0, 0, 1, 2); // → constrained to (1,2)
        let moves = g.legal_moves();
        assert!(moves.iter().all(|&(mr, mc, _, _)| (mr, mc) == (1, 2)));
    }

    #[test]
    fn legal_moves_skips_resolved_boards() {
        let mut g = Game::new();
        g.board.sub_boards[1][1].outcome = Outcome::Win(Mark::X);
        g.active_sub_board = None;
        let moves = g.legal_moves();
        assert!(moves.iter().all(|&(mr, mc, _, _)| (mr, mc) != (1, 1)));
    }

    #[test]
    fn legal_moves_empty_after_game_over() {
        let mut g = Game::new();
        g.board.outcome = Outcome::Win(Mark::X);
        assert!(g.legal_moves().is_empty());
    }

    // -- Game: meta-level win --

    #[test]
    fn game_detects_meta_row_win() {
        let mut g = Game::new();
        // Pre-win two sub-boards, then trigger the third via play()
        g.board.sub_boards[0][0].outcome = Outcome::Win(Mark::X);
        g.board.sub_boards[0][1].outcome = Outcome::Win(Mark::X);
        g.board.sub_boards[0][2].cells[0][0] = Mark::X;
        g.board.sub_boards[0][2].cells[0][1] = Mark::X;
        g.current_turn = Mark::X;
        g.active_sub_board = Some((0, 2));
        g.play(0, 2, 0, 2); // X completes row in sub-board (0,2)
        assert_eq!(g.outcome(), Outcome::Win(Mark::X));
    }
}
