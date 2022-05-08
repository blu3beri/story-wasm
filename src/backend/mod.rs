use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

pub mod simple;

#[wasm_bindgen(typescript_custom_section)]
const IBACKEND_OPTIONS: &'static str = r#"
interface IBackendOptions {
    path?: string
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IBackendOptions")]
    #[derive(Debug, Default)]
    pub type IBackendOptions;
}

#[wasm_bindgen]
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BackendOptions {
    path: Option<PathBuf>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Backend {
    Simple,
}

impl From<Backend> for String {
    fn from(backend: Backend) -> Self {
        match backend {
            Backend::Simple => String::from("simple"),
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Backend::Simple
    }
}

pub trait BackendExtended {
    fn new(options: &Option<BackendOptions>) -> Result<Self>
    where
        Self: Sized;
    fn get(&self, key: impl AsRef<str>) -> Result<Option<String>>;
    fn put(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<Option<String>>;
    fn delete(&self, key: impl AsRef<str>) -> Result<Option<String>>;
}
