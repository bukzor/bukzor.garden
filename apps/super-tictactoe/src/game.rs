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
