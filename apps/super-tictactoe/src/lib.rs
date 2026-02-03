use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

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
}

#[derive(Clone, Copy)]
struct Cell {
    row: usize,
    col: usize,
    mark: Mark,
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col, mark: Mark::Empty }
    }

    fn render(&self, document: &Document) -> Result<Element, JsValue> {
        let el = document.create_element("div")?;
        el.set_class_name("cell");
        el.set_attribute("data-row", &self.row.to_string())?;
        el.set_attribute("data-col", &self.col.to_string())?;
        el.set_text_content(Some(self.mark.symbol()));
        Ok(el)
    }
}

struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: std::array::from_fn(|i| Cell::new(i / 3, i % 3)),
        }
    }

    fn render(&self, document: &Document) -> Result<Element, JsValue> {
        let container = document.create_element("div")?;
        container.set_class_name("board");

        for cell in &self.cells {
            let cell_el: Element = cell.render(document)?;
            container.append_child(&cell_el)?;
        }

        Ok(container)
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

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let body = document.body().ok_or("no body")?;

    setup_head(&document)?;

    let board = Board::new();
    let board_el: Element = board.render(&document)?;
    body.append_child(&board_el)?;

    Ok(())
}
