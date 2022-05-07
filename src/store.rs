use crate::backend::IBackendOptions;
use crate::backend::{text::TextBackend, Backend, BackendExtended, BackendOptions};
use crate::error::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const IBACKEND_OPTIONS: &'static str = r#"
interface BackendOptions {
    path?: string
}
"#;

#[wasm_bindgen]
pub struct Store {
    backend: Backend,
    options: BackendOptions,
}

#[wasm_bindgen]
impl Store {
    pub fn new(backend: Backend, options: IBackendOptions) -> Self {
        let options: BackendOptions = options.into_serde().unwrap();
        log!("{:?}", options);
        Self { backend, options }
    }

    fn get_backend(&self) -> Result<impl BackendExtended> {
        match self.backend {
            Backend::Text => TextBackend::new(&self.options),
        }
    }

    pub fn get(&self, key: String) -> Result<String> {
        let backend = self.get_backend();

        Err(Error::InvalidBackendOptions("text".to_string(), "foo, bar".to_string()).into())
    }

    pub fn post(&self, key: String, value: String) {
        todo!()
    }

    pub fn put(&self, key: String, value: String) -> String {
        todo!()
    }

    pub fn delete(&self, key: String, value: String) -> String {
        todo!()
    }
}
