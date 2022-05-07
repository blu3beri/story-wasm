use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! log {
    ($($t:tt)*) => (crate::wasm::log(&format_args!($($t)*).to_string()))
}
