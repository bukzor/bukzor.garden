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

struct Game {
    board: [[Mark; 3]; 3],
    current_turn: Mark,
    outcome: Outcome,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[Mark::Empty; 3]; 3],
            current_turn: Mark::X,
            outcome: Outcome::InProgress,
        }
    }

    fn shared() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new()))
    }

    fn check_winner(&self) -> Option<Mark> {
        for line in WINNING_LINES {
            let [a, b, c] = line.map(|(r, c)| self.board[r][c]);
            if a != Mark::Empty && a == b && b == c {
                return Some(a);
            }
        }
        None
    }

    fn is_full(&self) -> bool {
        self.board.iter().flatten().all(|&m| m != Mark::Empty)
    }

    fn play(&mut self, row: usize, col: usize) -> bool {
        if self.outcome != Outcome::InProgress {
            return false;
        }
        if self.board[row][col] != Mark::Empty {
            return false;
        }
        self.board[row][col] = self.current_turn;
        if let Some(winner) = self.check_winner() {
            self.outcome = Outcome::Win(winner);
            self.current_turn = Mark::Empty;
        } else if self.is_full() {
            self.outcome = Outcome::Draw;
            self.current_turn = Mark::Empty;
        } else {
            self.current_turn = self.current_turn.next();
        }
        true
    }
}

fn render_cell(document: &Document, row: usize, col: usize, mark: Mark) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;
    el.set_class_name("cell");
    el.set_attribute("data-row", &row.to_string())?;
    el.set_attribute("data-col", &col.to_string())?;
    el.set_text_content(Some(mark.symbol()));
    Ok(el)
}

fn render_board(document: &Document, game: &Game) -> Result<Element, JsValue> {
    let board = document.create_element("div")?;
    board.set_class_name("board");

    for (row, marks) in game.board.iter().enumerate() {
        for (col, &mark) in marks.iter().enumerate() {
            let cell: Element = render_cell(document, row, col, mark)?;
            board.append_child(&cell)?;
        }
    }

    Ok(board)
}

fn cell_from_event(event: &web_sys::Event) -> Option<(Element, usize, usize)> {
    let el = event.target()?.dyn_ref::<Element>()?.clone();
    if !el.class_list().contains("cell") {
        return None;
    }
    let row = el.get_attribute("data-row")?.parse().ok()?;
    let col = el.get_attribute("data-col")?.parse().ok()?;
    Some((el, row, col))
}

fn on_board_click(board: &Element, game: Rc<RefCell<Game>>, status: Element) -> EventListener {
    EventListener::new(board, "click", move |event| {
        let Some((el, row, col)) = cell_from_event(event) else { return };

        let mut game = game.borrow_mut();
        if game.play(row, col) {
            el.set_text_content(Some(game.board[row][col].symbol()));
            match game.outcome {
                Outcome::Win(mark) => {
                    status.set_text_content(Some(&format!("{} wins!", mark.symbol())));
                }
                Outcome::Draw => {
                    status.set_text_content(Some("Draw!"));
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

        let status = document.create_element("div")?;
        status.set_class_name("status");
        body.append_child(&status)?;

        let board = render_board(&document, &game.borrow())?;
        let listener = on_board_click(&board, Rc::clone(&game), status);
        body.append_child(&board)?;

        Ok(App { game, _listener: listener })
    }

    #[wasm_bindgen(getter)]
    pub fn current_turn(&self) -> String {
        self.game.borrow().current_turn.symbol().to_string()
    }
}
