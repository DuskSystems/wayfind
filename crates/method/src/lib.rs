#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use errors::{DeleteError, InsertError, SearchError};
use id::MethodIdGenerator;
use std::collections::BTreeMap;

pub mod display;
pub mod errors;
pub mod id;

pub use id::MethodId;

// FIXME: Make the key here impl based, so we can make custom keys.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodMatch<'r> {
    pub id: MethodId,
    pub method: Option<&'r str>,
}

#[derive(Clone)]
pub struct MethodRouter {
    pub map: BTreeMap<usize, BTreeMap<String, MethodId>>,
    pub id: MethodIdGenerator,
}

impl MethodRouter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: BTreeMap::default(),
            id: MethodIdGenerator::default(),
        }
    }

    pub fn insert(&mut self, path_id: usize, methods: &[&str]) -> Result<MethodId, InsertError> {
        if methods.is_empty() {
            return Err(InsertError::Empty);
        }

        let map = self.map.entry(path_id).or_default();

        for method in methods {
            if map.contains_key(*method) {
                return Err(InsertError::Conflict);
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
        path_id: usize,
        method: &'r str,
    ) -> Result<Option<MethodMatch<'r>>, SearchError> {
        if let Some(method_map) = self.map.get(&path_id) {
            if let Some(id) = method_map.get(method) {
                return Ok(Some(MethodMatch {
                    id: *id,
                    method: Some(method),
                }));
            }

            return Err(SearchError::NotAllowed);
        }

        Ok(None)
    }

    pub fn find(&self, path_id: usize, methods: &[&str]) -> Result<MethodId, DeleteError> {
        if methods.is_empty() {
            return Ok(MethodId(None));
        }

        let map = self.map.get(&path_id).ok_or(DeleteError::NotFound)?;
        for method in methods {
            if !map.contains_key(*method) {
                return Err(DeleteError::Mismatch);
            }
        }

        let first_id = map.get(methods[0]).unwrap();
        for method in methods {
            match map.get(*method) {
                Some(id) if id == first_id => continue,
                _ => {
                    return Err(DeleteError::Mismatch);
                }
            }
        }

        if map
            .iter()
            .any(|(method, id)| id == first_id && !methods.contains(&method.as_str()))
        {
            return Err(DeleteError::Mismatch);
        }

        Ok(*first_id)
    }

    pub fn delete(&mut self, path_id: usize, method_id: MethodId) {
        if let Some(map) = self.map.get_mut(&path_id) {
            map.retain(|_, id| id != &method_id);
            if map.is_empty() {
                self.map.remove(&path_id);
            }
        }
    }
}
