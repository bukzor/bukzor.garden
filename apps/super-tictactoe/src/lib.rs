use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");

    body.set_inner_html("<h1>Super Tic-Tac-Toe</h1><p>Hello from WASM! (2)</p>");
}
