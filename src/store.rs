use crate::backend::Backend;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Store {
    backend: Backend,
}

#[wasm_bindgen]
impl Store {
    pub fn init(backend: Backend) -> Self {
        Self { backend }
    }

    pub fn get_backend(&self) -> String {
        self.backend.into()
    }
}
