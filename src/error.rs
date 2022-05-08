use js_sys::Error as JSError;
use wasm_bindgen::prelude::*;

pub enum Error {
    InvalidBackendOptions(&'static str, Vec<&'static str>),
    UnableToOpenFile(String),
    PoisonedLock(String),
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        match error {
            Error::UnableToOpenFile(reason) => {
                JSError::new(format!("Unable to open file. Reason: {}", reason).as_str()).into()
            }
            Error::PoisonedLock(reason) => JSError::new(
                format!("Read / Write lock has been poisoned. Reason: {}", reason).as_str(),
            )
            .into(),
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
