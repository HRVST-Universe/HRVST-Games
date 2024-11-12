use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Use `web_sys`'s console log function to output to the browser's console
    web_sys::console::log_1(&"WASM module loaded successfully".into());
    Ok(())
}
