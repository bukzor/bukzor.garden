use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, closure::Closure, JsCast};
use web_sys::{Document, Element, MouseEvent};

#[derive(Clone, Copy, PartialEq)]
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

    fn next_turn(self) -> Self {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::Empty => Mark::X,
        }
    }
}

#[derive(Clone, Copy)]
struct Cell {
    row: usize,
    col: usize,
    mark: Mark,
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            mark: Mark::Empty,
        }
    }

    fn render(&self, document: &Document) -> Result<Element, JsValue> {
        let el = document.create_element("div")?;
        el.set_class_name("cell");
        el.set_attribute("data-row", &self.row.to_string())?;
        el.set_attribute("data-col", &self.col.to_string())?;
        el.set_text_content(Some(self.mark.symbol()));
        Ok(el)
    }

    fn on_click(&self, game: &Rc<RefCell<Game>>) -> Closure<dyn Fn(MouseEvent)> {
        let game = Rc::clone(game);
        let (row, col) = (self.row, self.col);
        Closure::new(move |event: MouseEvent| {
            let Some(target) = event.target() else { return };
            let Ok(element) = target.dyn_into::<Element>() else { return };
            let mut game = game.borrow_mut();
            if game.play(row, col) {
                element.set_text_content(Some(game.cells[row][col].mark.symbol()));
            }
        })
    }
}

struct Game {
    cells: [[Cell; 3]; 3],
    current_turn: Mark,
}

impl Game {
    fn new() -> Self {
        Self {
            cells: std::array::from_fn(|row| std::array::from_fn(|col| Cell::new(row, col))),
            current_turn: Mark::X,
        }
    }

    fn play(&mut self, row: usize, col: usize) -> bool {
        if self.cells[row][col].mark != Mark::Empty {
            return false;
        }
        self.cells[row][col].mark = self.current_turn;
        self.current_turn = self.current_turn.next_turn();
        true
    }
}

fn setup_head(document: &Document) -> Result<(), JsValue> {
    let head = document.head().ok_or("no head")?;

    document.set_title("Tic-Tac-Toe");

    let charset = document.create_element("meta")?;
    charset.set_attribute("charset", "utf-8")?;
    head.append_child(&charset)?;

    let viewport = document.create_element("meta")?;
    viewport.set_attribute("name", "viewport")?;
    viewport.set_attribute("content", "width=device-width, initial-scale=1")?;
    head.append_child(&viewport)?;

    Ok(())
}

fn render_board(document: &Document, game: Rc<RefCell<Game>>) -> Result<Element, JsValue> {
    let container = document.create_element("div")?;
    container.set_class_name("board");

    for row in game.borrow().cells {
        for cell in row {
            let cell_el = cell.render(document)?;
            let closure = cell.on_click(&game);
            cell_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
            container.append_child(&cell_el)?;
        }
    }

    Ok(container)
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let body = document.body().ok_or("no body")?;

    setup_head(&document)?;

    let game = Rc::new(RefCell::new(Game::new()));
    let board_el = render_board(&document, game)?;
    body.append_child(&board_el)?;

    Ok(())
}
