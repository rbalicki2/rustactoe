#![feature(proc_macro_hygiene, slice_patterns)]

use wasm_bindgen::prelude::*;
use web_sys::console;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod game;

fn get_root_element() -> Result<web_sys::Element, JsValue> {
  web_sys::window()
    .and_then(|w| w.document())
    // N.B. query_selector returns Result<Option<Element>>
    // So, calling .ok() on that converts it to an Option<Option<Element>>
    // and hence, we must call .ok_or() twice.
    .and_then(|d| d.query_selector("#app").ok())
    .ok_or(JsValue::NULL)?
    .ok_or(JsValue::NULL)
}

// This is the entry point of your app
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  let root_element = get_root_element()?;

  let board = game::Board::empty();
  let app = app::render(board);

  smithy::mount(Box::new(app), root_element);

  Ok(())
}
