#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use errors::{MethodDeleteError, MethodInsertError, MethodSearchError};
use id::MethodIdGenerator;
use std::collections::BTreeMap;
use wayfind_storage::Storage;

pub mod display;
pub mod errors;
pub mod id;

pub use id::MethodId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodMatch<'r> {
    pub id: MethodId,
    pub method: Option<&'r str>,
}

#[derive(Clone)]
pub struct MethodRouter {
    pub map: Storage<BTreeMap<String, MethodId>>,
    pub id: MethodIdGenerator,
}

impl MethodRouter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: Storage::default(),
            id: MethodIdGenerator::default(),
        }
    }

    pub fn insert(
        &mut self,
        key: Option<usize>,
        methods: &[&str],
    ) -> Result<MethodId, MethodInsertError> {
        if methods.is_empty() {
            return Err(MethodInsertError::Empty);
        }

        if !self.map.contains(&key) {
            self.map.insert(key, BTreeMap::default());
        }

        let map = &mut self.map[&key];
        for method in methods {
            if map.contains_key(*method) {
                return Err(MethodInsertError::Conflict);
            }
        }

        let id = self.id.generate();
        for method in methods {
            map.insert((*method).to_owned(), id);
        }

        Ok(id)
    }

    pub fn search<'r>(
        &self,
        key: Option<usize>,
        method: &'r str,
    ) -> Result<Option<MethodMatch<'r>>, MethodSearchError> {
        if let Some(method_map) = self.map.get(&key) {
            if let Some(id) = method_map.get(method) {
                return Ok(Some(MethodMatch {
                    id: *id,
                    method: Some(method),
                }));
            }

            return Err(MethodSearchError::NotAllowed);
        }

        Ok(None)
    }

    pub fn find(
        &self,
        key: Option<usize>,
        methods: &[&str],
    ) -> Result<MethodId, MethodDeleteError> {
        if methods.is_empty() {
            return Ok(MethodId(None));
        }

        let map = self.map.get(&key).ok_or(MethodDeleteError::NotFound)?;
        for method in methods {
            if !map.contains_key(*method) {
                return Err(MethodDeleteError::Mismatch);
            }
        }

        let first_id = map.get(methods[0]).unwrap();
        for method in methods {
            match map.get(*method) {
                Some(id) if id == first_id => continue,
                _ => {
                    return Err(MethodDeleteError::Mismatch);
                }
            }
        }

        if map
            .iter()
            .any(|(method, id)| id == first_id && !methods.contains(&method.as_str()))
        {
            return Err(MethodDeleteError::Mismatch);
        }

        Ok(*first_id)
    }

    pub fn delete(&mut self, key: Option<usize>, method_id: MethodId) {
        if let Some(map) = self.map.get_mut(&key) {
            map.retain(|_, id| id != &method_id);
            if map.is_empty() {
                self.map.remove(&key);
            }
        }
    }
}
