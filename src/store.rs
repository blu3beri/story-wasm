use crate::backend::IBackendOptions;
use crate::backend::{btreemap::BTreeMapBackend, Backend, BackendExtended, BackendOptions};
use crate::error::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Store {
    backend: Backend,
    options: Option<BackendOptions>,
}

#[wasm_bindgen]
impl Store {
    pub fn new(backend: Backend, options: Option<IBackendOptions>) -> Self {
        let options: Option<BackendOptions> = match options {
            Some(o) => o.into_serde().unwrap(),
            None => None,
        };
        Self { backend, options }
    }

    fn get_backend(&self) -> Result<impl BackendExtended> {
        match self.backend {
            Backend::BTreeMap => BTreeMapBackend::new(&self.options),
        }
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let backend = self.get_backend()?;
        backend.get(key)
    }

    pub fn put(&self, key: String, value: String) -> Result<Option<String>> {
        let backend = self.get_backend()?;
        backend.put(key, value)
    }

    pub fn delete(&self, key: String) -> Result<Option<String>> {
        let backend = self.get_backend()?;
        backend.delete(key)
    }
}
