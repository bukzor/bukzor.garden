use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlInputElement};

use crate::ai_random;
use crate::game::*;

fn render_cell(
    document: &Document,
    meta_row: usize,
    meta_col: usize,
    row: usize,
    col: usize,
) -> Result<Element, wasm_bindgen::JsValue> {
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
) -> Result<Element, wasm_bindgen::JsValue> {
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

fn render_board(document: &Document, game: &Game) -> Result<Element, wasm_bindgen::JsValue> {
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

fn cell_from_event(event: &web_sys::Event) -> Option<(Element, Move)> {
    let el = event.target()?.dyn_ref::<Element>()?.clone();
    if !el.class_list().contains("cell") {
        return None;
    }
    let board = BoardPos {
        row: el.get_attribute("data-meta-row")?.parse().ok()?,
        col: el.get_attribute("data-meta-col")?.parse().ok()?,
    };
    let cell = CellPos {
        row: el.get_attribute("data-row")?.parse().ok()?,
        col: el.get_attribute("data-col")?.parse().ok()?,
    };
    Some((el, Move { board, cell }))
}

fn update_constraints(board_el: &Element, active_board: Option<BoardPos>) {
    let children = board_el.children();
    for i in 0..children.length() {
        let Some(sub) = children.item(i) else { continue };
        let Ok(Some(status)) = sub.query_selector(".status") else { continue };

        // Skip resolved boards - they keep their resolved styling
        if status.has_attribute("data-resolved") {
            continue;
        }

        let pos = BoardPos {
            row: sub.get_attribute("data-meta-row").and_then(|s: String| s.parse().ok()).unwrap_or(99),
            col: sub.get_attribute("data-meta-col").and_then(|s: String| s.parse().ok()).unwrap_or(99),
        };

        match active_board {
            Some(active) if pos != active => {
                let _ = status.set_attribute("data-constrained", "");
            }
            _ => {
                let _ = status.remove_attribute("data-constrained");
            }
        }
    }
}

fn find_cell(board_el: &Element, mov: Move) -> Option<Element> {
    let selector = format!(
        ".cell[data-meta-row='{}'][data-meta-col='{}'][data-row='{}'][data-col='{}']",
        mov.board.row, mov.board.col, mov.cell.row, mov.cell.col
    );
    board_el.query_selector(&selector).ok().flatten()
}

pub struct AutoPlay {
    pub x: HtmlInputElement,
    pub o: HtmlInputElement,
    pub delay: HtmlInputElement,
}

impl AutoPlay {
    pub fn from_document(document: &Document) -> Result<Self, wasm_bindgen::JsValue> {
        let get_input = |id: &str| -> Result<HtmlInputElement, wasm_bindgen::JsValue> {
            document
                .get_element_by_id(id)
                .ok_or_else(|| wasm_bindgen::JsValue::from_str(&format!("missing #{}", id)))?
                .dyn_into()
                .map_err(|_| wasm_bindgen::JsValue::from_str(&format!("#{} is not an input", id)))
        };
        Ok(Self {
            x: get_input("auto-x")?,
            o: get_input("auto-o")?,
            delay: get_input("delay")?,
        })
    }

    pub fn is_enabled(&self, mark: Mark) -> bool {
        match mark {
            Mark::X => self.x.checked(),
            Mark::O => self.o.checked(),
            Mark::Empty => false,
        }
    }

    pub fn delay_ms(&self) -> u32 {
        self.delay.value().parse().unwrap_or(1500)
    }
}

pub struct Ui {
    pub game: RefCell<Game>,
    pub board_el: Element,
    pub final_status: Element,
    pub turn_indicator: Element,
    pub auto_play: AutoPlay,
}

impl Ui {
    pub fn new(document: &Document) -> Result<Self, wasm_bindgen::JsValue> {
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

    pub fn update_turn_indicator(turn_indicator: &Element, current_turn: Mark) {
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

    pub fn schedule_auto_play(self: &Rc<Self>) {
        let game = self.game.borrow();
        let next_turn = game.current_turn;
        if game.outcome() != Outcome::InProgress || !self.auto_play.is_enabled(next_turn) {
            return;
        }
        let moves = game.legal_moves();
        let Some(&mov) = ai_random::pick_random(&moves) else { return };
        drop(game);

        let delay = self.auto_play.delay_ms();
        let ui = Rc::clone(self);
        Timeout::new(delay, move || {
            if !ui.auto_play.is_enabled(next_turn) {
                return;
            }
            if let Some(cell) = find_cell(&ui.board_el, mov) {
                if let Ok(html_el) = cell.dyn_into::<web_sys::HtmlElement>() {
                    html_el.click();
                }
            }
        })
        .forget();
    }

    pub fn resolve_all_sub_boards(&self) {
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

    pub fn handle_click(self: &Rc<Self>, event: &web_sys::Event) {
        let Some((el, mov)) = cell_from_event(event) else { return };

        let mut game = self.game.borrow_mut();
        if game.play(mov) {
            let sub = game.sub_board(mov.board);
            let mark = sub.cells[mov.cell.row][mov.cell.col];
            let _ = el.set_attribute("data-mark", mark.symbol());

            if sub.outcome != Outcome::InProgress {
                if let Some(sub_board_el) = el.parent_element() {
                    if let Ok(Some(status_el)) = sub_board_el.query_selector(".status") {
                        let _ = status_el.set_attribute("data-resolved", "");
                        if let Outcome::Win(winner) = sub.outcome {
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
