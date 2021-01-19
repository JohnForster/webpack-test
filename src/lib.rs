use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo(n: &str) -> JsValue {
    JsValue::from(format!("Hello from rust, {}", n))
}
