mod app;
mod render;

use app::App;
// use render::render_to_file;

use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
    // render_to_file();
}

/*

Taken from: https://medium.com/@syntaxSavage/the-night-i-realized-my-web-apps-didnt-need-servers-anymore-5e6418d0e33e

Rust Code

// src/lib.rs
use wasm_bindgen::prelude::*;
use pulldown_cmark::{html, Parser};


#[wasm_bindgen]
pub fn render_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

Build

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/markdown_renderer.wasm \
  --out-dir ./pkg --target web

Browser Integration

<textarea id="editor"># Hello World</textarea>
<div id="preview"></div>


<script type="module">
  import init, { render_markdown } from "./pkg/markdown_renderer.js";
  async function start() {
    await init();
    const editor = document.getElementById("editor");
    const preview = document.getElementById("preview");
    editor.addEventListener("input", () => {
      preview.innerHTML = render_markdown(editor.value);
    });
    preview.innerHTML = render_markdown(editor.value);
  }
  start();
</script>
*/
