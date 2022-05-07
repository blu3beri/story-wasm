use wasm_bindgen::prelude::*;

pub enum Error {
    InvalidBackendOptions(String, String),
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidBackendOptions(backend, missing) => JsValue::from_str(
                format!(
                    "Missing the following required options for the backend: {}. {}",
                    backend, missing
                )
                .as_str(),
            ),
        }
    }
}

pub type Result<T> = std::result::Result<T, JsValue>;
