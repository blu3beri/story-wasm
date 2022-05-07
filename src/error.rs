use js_sys::Error as JSError;
use wasm_bindgen::prelude::*;

pub enum Error {
    InvalidBackendOptions(&'static str, Vec<&'static str>),
    Test,
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        match error {
            Error::Test => JSError::new("foo").into(),
            Error::InvalidBackendOptions(backend, missing) => JSError::new(
                format!(
                    "Backend '{}' requires the following properties: {}",
                    backend,
                    missing.join(", ")
                )
                .as_str(),
            )
            .into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, JsValue>;
