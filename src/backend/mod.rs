use crate::error::*;
use wasm_bindgen::prelude::*;

pub mod text;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Backend {
    Text,
}

impl From<Backend> for String {
    fn from(backend: Backend) -> Self {
        match backend {
            Backend::Text => String::from("text"),
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Backend::Text
    }
}

pub trait BackendExtended {
    fn get(key: impl AsRef<str>) -> Result<String>;
    fn post(key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()>;
    fn put(key: impl AsRef<str>, value: impl AsRef<str>) -> Result<String>;
    fn delete(key: impl AsRef<str>, value: impl AsRef<str>) -> Result<String>;
}
