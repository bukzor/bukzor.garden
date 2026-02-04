use std::{cell::RefCell, rc::Rc};
use gloo_events::EventListener;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mark {
    Empty,
    X,
    O,
}

impl Mark {
    fn symbol(self) -> &'static str {
        match self {
            Mark::Empty => "",
            Mark::X => "X",
            Mark::O => "O",
        }
    }

    fn next(self) -> Self {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::Empty => Mark::X,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
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

const WINNING_LINES: [[(usize, usize); 3]; 8] = winning_lines();

fn check_winner<T: Copy + PartialEq>(
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
struct SubBoard {
    cells: [[Mark; 3]; 3],
    outcome: Outcome,
}

impl SubBoard {
    fn new() -> Self {
        Self {
            cells: [[Mark::Empty; 3]; 3],
            outcome: Outcome::InProgress,
        }
    }

    fn check_winner(&self) -> Option<Mark> {
        check_winner(&self.cells, |m| (m != Mark::Empty).then_some(m))
    }

    fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|&m| m != Mark::Empty)
    }

    fn play(&mut self, row: usize, col: usize, mark: Mark) -> bool {
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

struct Game {
    boards: [[SubBoard; 3]; 3],
    current_turn: Mark,
    outcome: Outcome,
    active_board: Option<(usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Self {
            boards: [[SubBoard::new(); 3]; 3],
            current_turn: Mark::X,
            outcome: Outcome::InProgress,
            active_board: None,
        }
    }

    fn shared() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new()))
    }

    fn check_winner(&self) -> Option<Mark> {
        check_winner(&self.boards, |sb| match sb.outcome {
            Outcome::Win(mark) => Some(mark),
            _ => None,
        })
    }

    fn is_full(&self) -> bool {
        self.boards
            .iter()
            .flatten()
            .all(|sb| sb.outcome != Outcome::InProgress)
    }

    fn play(&mut self, meta_row: usize, meta_col: usize, row: usize, col: usize) -> bool {
        if self.outcome != Outcome::InProgress {
            return false;
        }
        if let Some((ar, ac)) = self.active_board {
            if (meta_row, meta_col) != (ar, ac) {
                return false;
            }
        }
        let sub_board = &mut self.boards[meta_row][meta_col];
        if !sub_board.play(row, col, self.current_turn) {
            return false;
        }
        if let Some(winner) = self.check_winner() {
            self.outcome = Outcome::Win(winner);
            self.current_turn = Mark::Empty;
            self.active_board = None;
        } else if self.is_full() {
            self.outcome = Outcome::Draw;
            self.current_turn = Mark::Empty;
            self.active_board = None;
        } else {
            self.current_turn = self.current_turn.next();
            let target = &self.boards[row][col];
            self.active_board = if target.outcome == Outcome::InProgress {
                Some((row, col))
            } else {
                None
            };
        }
        true
    }
}

fn render_cell(
    document: &Document,
    meta_row: usize,
    meta_col: usize,
    row: usize,
    col: usize,
    mark: Mark,
) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;
    el.set_class_name("cell");
    el.set_attribute("data-meta-row", &meta_row.to_string())?;
    el.set_attribute("data-meta-col", &meta_col.to_string())?;
    el.set_attribute("data-row", &row.to_string())?;
    el.set_attribute("data-col", &col.to_string())?;
    el.set_text_content(Some(mark.symbol()));
    Ok(el)
}

fn render_sub_board(
    document: &Document,
    meta_row: usize,
    meta_col: usize,
    sub_board: &SubBoard,
) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;
    el.set_class_name("sub-board");
    el.set_attribute("data-meta-row", &meta_row.to_string())?;
    el.set_attribute("data-meta-col", &meta_col.to_string())?;

    for (row, marks) in sub_board.cells.iter().enumerate() {
        for (col, &mark) in marks.iter().enumerate() {
            let cell = render_cell(document, meta_row, meta_col, row, col, mark)?;
            el.append_child(&cell)?;
        }
    }

    let status = document.create_element("div")?;
    status.set_class_name("status");
    el.append_child(&status)?;

    Ok(el)
}

fn render_board(document: &Document, game: &Game) -> Result<Element, JsValue> {
    let board = document.create_element("div")?;
    board.set_class_name("board");

    for (meta_row, row_boards) in game.boards.iter().enumerate() {
        for (meta_col, sub_board) in row_boards.iter().enumerate() {
            let sub = render_sub_board(document, meta_row, meta_col, sub_board)?;
            board.append_child(&sub)?;
        }
    }

    Ok(board)
}

fn cell_from_event(event: &web_sys::Event) -> Option<(Element, usize, usize, usize, usize)> {
    let el = event.target()?.dyn_ref::<Element>()?.clone();
    if !el.class_list().contains("cell") {
        return None;
    }
    let meta_row = el.get_attribute("data-meta-row")?.parse().ok()?;
    let meta_col = el.get_attribute("data-meta-col")?.parse().ok()?;
    let row = el.get_attribute("data-row")?.parse().ok()?;
    let col = el.get_attribute("data-col")?.parse().ok()?;
    Some((el, meta_row, meta_col, row, col))
}

fn update_constraints(board_el: &Element, active_board: Option<(usize, usize)>) {
    let children = board_el.children();
    for i in 0..children.length() {
        let Some(sub) = children.item(i) else { continue };
        let Ok(Some(status)) = sub.query_selector(".status") else { continue };

        // Skip resolved boards - they keep their resolved styling
        if status.has_attribute("data-resolved") {
            continue;
        }

        let mr: usize = sub.get_attribute("data-meta-row").and_then(|s: String| s.parse().ok()).unwrap_or(99);
        let mc: usize = sub.get_attribute("data-meta-col").and_then(|s: String| s.parse().ok()).unwrap_or(99);

        match active_board {
            Some((ar, ac)) if (mr, mc) != (ar, ac) => {
                let _ = status.set_attribute("data-constrained", "");
            }
            _ => {
                let _ = status.remove_attribute("data-constrained");
            }
        }
    }
}

fn on_board_click(board_el: &Element, game: Rc<RefCell<Game>>, final_status: Element) -> EventListener {
    EventListener::new(board_el, "click", move |event| {
        let Some((el, meta_row, meta_col, row, col)) = cell_from_event(event) else { return };

        let mut game = game.borrow_mut();
        if game.play(meta_row, meta_col, row, col) {
            el.set_text_content(Some(game.boards[meta_row][meta_col].cells[row][col].symbol()));

            let sub_outcome = game.boards[meta_row][meta_col].outcome;
            if sub_outcome != Outcome::InProgress {
                if let Some(sub_board_el) = el.parent_element() {
                    if let Ok(Some(status_el)) = sub_board_el.query_selector(".status") {
                        let _ = status_el.set_attribute("data-resolved", "");
                        if let Outcome::Win(winner) = sub_outcome {
                            status_el.set_text_content(Some(winner.symbol()));
                        }
                    }
                }
            }

            // Navigate: cell → sub-board → board
            if let Some(sub_board_el) = el.parent_element() {
                if let Some(board_el) = sub_board_el.parent_element() {
                    update_constraints(&board_el, game.active_board);
                }
            }

            match game.outcome {
                Outcome::Win(mark) => {
                    final_status.set_text_content(Some(&format!("{} wins!", mark.symbol())));
                }
                Outcome::Draw => {
                    final_status.set_text_content(Some("Draw!"));
                }
                Outcome::InProgress => {}
            }
        }
    })
}

#[wasm_bindgen]
pub struct App {
    game: Rc<RefCell<Game>>,
    _listener: EventListener,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(document: Document) -> Result<App, JsValue> {
        let body = document.body().ok_or("no body")?;

        let game = Game::shared();

        let final_status = document.create_element("div")?;
        final_status.set_class_name("final-status");
        body.append_child(&final_status)?;

        let board = render_board(&document, &game.borrow())?;
        let listener = on_board_click(&board, Rc::clone(&game), final_status);
        body.append_child(&board)?;

        Ok(App { game, _listener: listener })
    }

    #[wasm_bindgen(getter)]
    pub fn current_turn(&self) -> String {
        self.game.borrow().current_turn.symbol().to_string()
    }
}
