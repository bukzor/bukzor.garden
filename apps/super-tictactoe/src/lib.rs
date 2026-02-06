use std::rc::Rc;

use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Document;

pub mod ai_random;
pub mod game;
pub mod ui;

#[cfg(test)]
mod game_builder;

use ui::Ui;

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
