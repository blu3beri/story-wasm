use crate::error::Result;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod text;

#[wasm_bindgen(typescript_custom_section)]
const IBACKEND_OPTIONS: &'static str = r#"
interface IBackendOptions {
    path?: string
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IBackendOptions")]
    #[derive(Debug)]
    pub type IBackendOptions;
}

#[wasm_bindgen]
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BackendOptions {
    path: Option<String>,
}

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
    fn new(options: &BackendOptions) -> Result<Self>
    where
        Self: Sized;
    fn get(&self, key: impl AsRef<str>) -> Result<String>;
    fn post(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()>;
    fn put(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()>;
    fn delete(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<()>;
}
