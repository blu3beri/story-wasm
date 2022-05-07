use super::{BackendExtended, BackendOptions};
use crate::error::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TextBackend;

impl BackendExtended for TextBackend {
    fn new(options: &BackendOptions) -> Result<Self> {
        Ok(TextBackend)
    }

    fn get(&self, key: impl AsRef<str>) -> Result<String> {
        todo!()
    }

    fn post(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        todo!()
    }

    fn put(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        todo!()
    }

    fn delete(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        todo!()
    }
}
