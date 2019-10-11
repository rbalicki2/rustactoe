#![feature(proc_macro_hygiene, slice_patterns)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod game;

use wasm_bindgen::prelude::wasm_bindgen;

fn get_root_element() -> Result<web_sys::Element, wasm_bindgen::JsValue> {
  web_sys::window()
    .and_then(|w| w.document())
    // N.B. query_selector returns Result<Option<Element>>
    // So, calling .ok() on that converts it to an Option<Option<Element>>
    // and hence, we must call .ok_or() twice.
    .and_then(|d| d.query_selector("#app").ok())
    .ok_or(wasm_bindgen::JsValue::NULL)?
    .ok_or(wasm_bindgen::JsValue::NULL)
}

// This is the entry point of your app
#[wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
  use smithy::types::Component;
  use smithy::*;
  
  // use smithy_types::core::AsInnerHtml
  console_error_panic_hook::set_once();

  // let root_element = get_root_element()?;

  let board = game::Board::empty();
  let mut app = app::render(board);

  let node = app.render();
  let collapsed_node_vec: Vec<smithy::types::CollapsedNode> = node.into();

  web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&collapsed_node_vec.as_inner_html()));

  // smithy::mount(Box::new(app), root_element);

  Ok(())
}
