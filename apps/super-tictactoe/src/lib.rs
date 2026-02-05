use std::{cell::RefCell, rc::Rc};
use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, HtmlInputElement};

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

struct Board {
    sub_boards: [[SubBoard; 3]; 3],
    outcome: Outcome,
}

impl Board {
    fn new() -> Self {
        Self {
            sub_boards: [[SubBoard::new(); 3]; 3],
            outcome: Outcome::InProgress,
        }
    }
}

struct Game {
    board: Board,
    current_turn: Mark,
    active_sub_board: Option<(usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board::new(),
            current_turn: Mark::X,
            active_sub_board: None,
        }
    }

    fn check_winner(&self) -> Option<Mark> {
        check_winner(&self.board.sub_boards, |sb| match sb.outcome {
            Outcome::Win(mark) => Some(mark),
            _ => None,
        })
    }

    fn is_full(&self) -> bool {
        self.board.sub_boards
            .iter()
            .flatten()
            .all(|sb| sb.outcome != Outcome::InProgress)
    }

    fn outcome(&self) -> Outcome {
        self.board.outcome
    }

    fn play(&mut self, meta_row: usize, meta_col: usize, row: usize, col: usize) -> bool {
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

    fn legal_moves(&self) -> Vec<(usize, usize, usize, usize)> {
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

fn render_cell(
    document: &Document,
    meta_row: usize,
    meta_col: usize,
    row: usize,
    col: usize,
) -> Result<Element, JsValue> {
    let el = document.create_element("div")?;
    el.set_class_name("cell");
    el.set_attribute("data-meta-row", &meta_row.to_string())?;
    el.set_attribute("data-meta-col", &meta_col.to_string())?;
    el.set_attribute("data-row", &row.to_string())?;
    el.set_attribute("data-col", &col.to_string())?;
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
        for (col, _) in marks.iter().enumerate() {
            let cell = render_cell(document, meta_row, meta_col, row, col)?;
            el.append_child(&cell)?;
        }
    }

    let status = document.create_element("div")?;
    status.set_class_name("status");
    el.append_child(&status)?;

    Ok(el)
}

fn render_board(document: &Document, game: &Game) -> Result<Element, JsValue> {
    let board_el = document.create_element("div")?;
    board_el.set_class_name("board");

    for (meta_row, row_boards) in game.board.sub_boards.iter().enumerate() {
        for (meta_col, sub_board) in row_boards.iter().enumerate() {
            let sub = render_sub_board(document, meta_row, meta_col, sub_board)?;
            board_el.append_child(&sub)?;
        }
    }

    Ok(board_el)
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

struct AutoPlay {
    x: HtmlInputElement,
    o: HtmlInputElement,
    delay: HtmlInputElement,
}

impl AutoPlay {
    fn from_document(document: &Document) -> Result<Self, JsValue> {
        let get_input = |id: &str| -> Result<HtmlInputElement, JsValue> {
            document
                .get_element_by_id(id)
                .ok_or_else(|| JsValue::from_str(&format!("missing #{}", id)))?
                .dyn_into()
                .map_err(|_| JsValue::from_str(&format!("#{} is not an input", id)))
        };
        Ok(Self {
            x: get_input("auto-x")?,
            o: get_input("auto-o")?,
            delay: get_input("delay")?,
        })
    }

    fn is_enabled(&self, mark: Mark) -> bool {
        match mark {
            Mark::X => self.x.checked(),
            Mark::O => self.o.checked(),
            Mark::Empty => false,
        }
    }

    fn delay_ms(&self) -> u32 {
        self.delay.value().parse().unwrap_or(1500)
    }
}

fn find_cell(board_el: &Element, mr: usize, mc: usize, r: usize, c: usize) -> Option<Element> {
    let selector = format!(
        ".cell[data-meta-row='{}'][data-meta-col='{}'][data-row='{}'][data-col='{}']",
        mr, mc, r, c
    );
    board_el.query_selector(&selector).ok().flatten()
}

struct Ui {
    game: RefCell<Game>,
    board_el: Element,
    final_status: Element,
    turn_indicator: Element,
    auto_play: AutoPlay,
}

impl Ui {
    fn new(document: &Document) -> Result<Self, JsValue> {
        let body = document.body().ok_or("no body")?;

        let game = RefCell::new(Game::new());

        let turn_indicator = document.create_element("div")?;
        turn_indicator.set_class_name("turn-indicator");

        let panel_x = document.create_element("div")?;
        panel_x.set_class_name("player-panel");
        panel_x.set_attribute("data-mark", "X")?;
        turn_indicator.append_child(&panel_x)?;

        let final_status = document.create_element("div")?;
        final_status.set_class_name("final-status");
        turn_indicator.append_child(&final_status)?;

        let panel_o = document.create_element("div")?;
        panel_o.set_class_name("player-panel");
        panel_o.set_attribute("data-mark", "O")?;
        turn_indicator.append_child(&panel_o)?;

        Self::update_turn_indicator(&turn_indicator, game.borrow().current_turn);
        body.append_child(&turn_indicator)?;

        let board_el = render_board(document, &game.borrow())?;
        body.append_child(&board_el)?;

        let auto_play = AutoPlay::from_document(document)?;

        Ok(Ui { game, board_el, final_status, turn_indicator, auto_play })
    }

    fn update_turn_indicator(turn_indicator: &Element, current_turn: Mark) {
        let panels = turn_indicator.children();
        for i in 0..panels.length() {
            let Some(panel) = panels.item(i) else { continue };
            let is_active = panel.get_attribute("data-mark").as_deref() == Some(current_turn.symbol());
            if is_active {
                let _ = panel.set_attribute("data-active", "");
            } else {
                let _ = panel.remove_attribute("data-active");
            }
        }
    }

    fn pick_random<T>(items: &[T]) -> Option<&T> {
        if items.is_empty() {
            return None;
        }
        let idx = (js_sys::Math::random() * items.len() as f64) as usize;
        items.get(idx.min(items.len() - 1))
    }

    fn schedule_auto_play(self: &Rc<Self>) {
        let game = self.game.borrow();
        let next_turn = game.current_turn;
        if game.outcome() != Outcome::InProgress || !self.auto_play.is_enabled(next_turn) {
            return;
        }
        let moves = game.legal_moves();
        let Some(&(mr, mc, r, c)) = Self::pick_random(&moves) else { return };
        drop(game);

        let delay = self.auto_play.delay_ms();
        let ui = Rc::clone(self);
        Timeout::new(delay, move || {
            if !ui.auto_play.is_enabled(next_turn) {
                return;
            }
            if let Some(cell) = find_cell(&ui.board_el, mr, mc, r, c) {
                if let Ok(html_el) = cell.dyn_into::<web_sys::HtmlElement>() {
                    html_el.click();
                }
            }
        })
        .forget();
    }

    fn resolve_all_sub_boards(&self) {
        let children = self.board_el.children();
        for i in 0..children.length() {
            let Some(sub) = children.item(i) else { continue };
            let Ok(Some(status)) = sub.query_selector(".status") else { continue };
            if !status.has_attribute("data-resolved") {
                let _ = status.set_attribute("data-resolved", "");
                let _ = status.remove_attribute("data-constrained");
            }
        }
    }

    fn handle_click(self: &Rc<Self>, event: &web_sys::Event) {
        let Some((el, meta_row, meta_col, row, col)) = cell_from_event(event) else { return };

        let mut game = self.game.borrow_mut();
        if game.play(meta_row, meta_col, row, col) {
            let mark = game.board.sub_boards[meta_row][meta_col].cells[row][col];
            let _ = el.set_attribute("data-mark", mark.symbol());

            let sub_outcome = game.board.sub_boards[meta_row][meta_col].outcome;
            if sub_outcome != Outcome::InProgress {
                if let Some(sub_board_el) = el.parent_element() {
                    if let Ok(Some(status_el)) = sub_board_el.query_selector(".status") {
                        let _ = status_el.set_attribute("data-resolved", "");
                        if let Outcome::Win(winner) = sub_outcome {
                            let _ = status_el.set_attribute("data-mark", winner.symbol());
                        }
                    }
                }
            }

            update_constraints(&self.board_el, game.active_sub_board);

            Self::update_turn_indicator(&self.turn_indicator, game.current_turn);

            match game.outcome() {
                Outcome::Win(mark) => {
                    self.final_status.set_text_content(Some(&format!("{} wins!", mark.symbol())));
                    self.resolve_all_sub_boards();
                }
                Outcome::Draw => {
                    self.final_status.set_text_content(Some("Draw!"));
                    self.resolve_all_sub_boards();
                }
                Outcome::InProgress => {}
            }

            drop(game);
            self.schedule_auto_play();
        }
    }
}

#[wasm_bindgen]
pub struct App {
    ui: Rc<Ui>,
    _listeners: Vec<EventListener>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(document: Document) -> Result<App, JsValue> {
        let ui = Rc::new(Ui::new(&document)?);

        let ui_click = Rc::clone(&ui);
        let click_listener = EventListener::new(&ui.board_el, "click", move |event| {
            ui_click.handle_click(event);
        });

        let ui_auto_x = Rc::clone(&ui);
        let auto_x_listener = EventListener::new(&ui.auto_play.x, "change", move |_| {
            ui_auto_x.schedule_auto_play();
        });

        let ui_auto_o = Rc::clone(&ui);
        let auto_o_listener = EventListener::new(&ui.auto_play.o, "change", move |_| {
            ui_auto_o.schedule_auto_play();
        });

        Ok(App {
            ui,
            _listeners: vec![click_listener, auto_x_listener, auto_o_listener],
        })
    }

    #[wasm_bindgen(getter)]
    pub fn current_turn(&self) -> String {
        self.ui.game.borrow().current_turn.symbol().to_string()
    }
}
