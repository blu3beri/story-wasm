use std::{collections::BTreeMap, sync::RwLock};

use super::{BackendExtended, BackendOptions};
use crate::error::*;

lazy_static! {
    static ref BTREEMAP: RwLock<BTreeMap<String, String>> = RwLock::new(BTreeMap::new());
}

pub struct BTreeMapBackend;

impl BackendExtended for BTreeMapBackend {
    fn new(_options: &Option<BackendOptions>) -> Result<Self> {
        Ok(BTreeMapBackend)
    }

    fn get(&self, key: impl AsRef<str>) -> Result<Option<String>> {
        let lock = BTREEMAP
            .read()
            .map_err(|e| Error::PoisonedLock(e.to_string()))?;
        Ok(lock.get(key.as_ref()).map(|v| v.to_string()))
    }

    fn put(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<Option<String>> {
        let mut lock = BTREEMAP
            .write()
            .map_err(|e| Error::PoisonedLock(e.to_string()))?;
        Ok(lock.insert(key.as_ref().to_string(), value.as_ref().to_string()))
    }

    fn delete(&self, key: impl AsRef<str>) -> Result<Option<String>> {
        let mut lock = BTREEMAP
            .write()
            .map_err(|e| Error::PoisonedLock(e.to_string()))?;
        Ok(lock.remove(&key.as_ref().to_string()))
    }
}
