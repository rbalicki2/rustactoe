#![feature(proc_macro_hygiene, slice_patterns)]

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod game;

// use wasm_bindgen::prelude::wasm_bindgen;

// fn get_root_element() -> Result<web_sys::Element, wasm_bindgen::JsValue> {
//   web_sys::window()
//     .and_then(|w| w.document())
//     // N.B. query_selector returns Result<Option<Element>>
//     // So, calling .ok() on that converts it to an Option<Option<Element>>
//     // and hence, we must call .ok_or() twice.
//     .and_then(|d| d.query_selector("#app").ok())
//     .ok_or(wasm_bindgen::JsValue::NULL)?
//     .ok_or(wasm_bindgen::JsValue::NULL)
// }

// This is the entry point of your app
// #[wasm_bindgen(start)]
// pub fn start() -> Result<(), wasm_bindgen::JsValue> {

use smithy::*;
fn print_to_console(app: &mut smithy::types::SmithyComponent) {
  let node = app.render();
  let collapsed_node_vec: Vec<smithy::types::CollapsedNode> = node.into();

  // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&collapsed_node_vec.as_inner_html()));
  println!("{}", collapsed_node_vec.as_inner_html());
  println!("\n\n----\n\n");
}

struct Terminal {
  nodes: Option<Vec<CollapsedNode>>,
}

trait AsTerminalText {
  fn as_terminal_text(&self) -> String;
}

impl AsTerminalText for Vec<CollapsedNode> {
  fn as_terminal_text(&self) -> String {
    self.iter().map(|node| node.as_terminal_text()).collect()
  }
}

impl AsTerminalText for CollapsedNode {
  fn as_terminal_text(&self) -> String {
    match self {
      CollapsedNode::Dom(token) => {
        token.as_terminal_text()
        // token.as_inner_html()
      },
      CollapsedNode::Text(s) => s.to_string(),
      CollapsedNode::Comment(str_opt) => match str_opt {
        Some(s) => format!("<!-- {} -->", s),
        None => "<!-- -->".into(),
      },
    }
  }
}

impl AsTerminalText for smithy::types::CollapsedHtmlToken {
  fn as_terminal_text(&self) -> String {
    let child_str = self
      .children
      .iter()
      .map(|child| child.as_terminal_text())
      .collect::<String>();
    if let Some(_) = self.attributes.get("red") {
      format!(
        "{}{}{}",
        termion::color::Fg(termion::color::Red),
        child_str,
        termion::style::Reset
      )
    } else {
      child_str
    }
    .to_string()
  }
}

impl RenderingTarget for Terminal {
  fn render(&mut self, nodes: Vec<CollapsedNode>) {
    print!("{}", termion::clear::All);
    // println!("{}", nodes.as_inner_html());

    println!("{}", nodes.as_terminal_text());

    // println!("----\n\n");
    self.nodes = Some(nodes);
  }

  fn apply_diff(&mut self, nodes: Vec<CollapsedNode>) {
    // println!("applied diff");
    self.render(nodes);
  }

  fn attach_listeners(&self) {}
}

pub fn main() {
  use smithy::types::Component;

  // use smithy_types::core::AsInnerHtml
  // console_error_panic_hook::set_once();

  // let root_element = get_root_element()?;

  // let board = game::Board::empty();
  // let mut app = app::render(board);

  let mut count = 0;
  let mut app = smithy::smd!(
    <text on_test={|_| count += 1}>count: <text red>{ count }</text></text>
  );

  // let node = app.render();
  // let collapsed_node_vec: Vec<smithy::types::CollapsedNode> = node.into();

  // // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&collapsed_node_vec.as_inner_html()));
  // println!("{}", collapsed_node_vec.as_inner_html());
  // print_to_console(&mut app);

  let terminal = Terminal { nodes: None };
  smithy::mount(Box::new(app), Box::new(terminal));
  // println!("\n\n----\n\n");

  smithy::handle_ui_event(&smithy::types::UiEvent::OnTest(true), &vec![0]);
  smithy::rerender();

  // print_to_console(&mut app);

  // app.handle_ui_event()

  // Ok(())
}
