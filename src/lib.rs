use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
pub fn foo(n: &HtmlElement) {
    web_sys::console::log_1(&n.tag_name().into());
}
