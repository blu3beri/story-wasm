use super::BackendExtended;
use crate::error::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct TextBackend;

impl BackendExtended for TextBackend {
    fn get(key: impl AsRef<str>) -> Result<String> {
        todo!()
    }

    fn post(key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        todo!()
    }

    fn put(key: impl AsRef<str>, value: impl AsRef<str>) -> Result<String> {
        todo!()
    }

    fn delete(key: impl AsRef<str>, value: impl AsRef<str>) -> Result<String> {
        todo!()
    }
}
