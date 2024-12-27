use crate::PathId;
use errors::{MethodDeleteError, MethodInsertError, MethodSearchError};
use id::MethodIdGenerator;
use std::collections::BTreeMap;

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
    pub map: BTreeMap<PathId, BTreeMap<String, MethodId>>,
    pub id: MethodIdGenerator,
}

impl MethodRouter {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::default(),
            id: MethodIdGenerator::default(),
        }
    }

    pub fn insert(
        &mut self,
        path_id: PathId,
        methods: &[&str],
    ) -> Result<MethodId, MethodInsertError> {
        if methods.is_empty() {
            return Err(MethodInsertError::Empty);
        }

        let map = self.map.entry(path_id).or_default();

        for method in methods {
            if map.contains_key(*method) {
                return Err(MethodInsertError::Conflict);
            }
        }

        let id = self.id.next();
        for method in methods {
            map.insert((*method).to_owned(), id);
        }

        Ok(id)
    }

    pub fn search<'r>(
        &self,
        path_id: PathId,
        method: &'r str,
    ) -> Result<Option<MethodMatch<'r>>, MethodSearchError> {
        if let Some(method_map) = self.map.get(&path_id) {
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

    pub fn find(&self, path_id: PathId, methods: &[&str]) -> Result<MethodId, MethodDeleteError> {
        if methods.is_empty() {
            return Ok(MethodId(None));
        }

        let map = self.map.get(&path_id).ok_or(MethodDeleteError::NotFound)?;
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

    pub fn delete(&mut self, path_id: PathId, method_id: MethodId) {
        if let Some(map) = self.map.get_mut(&path_id) {
            map.retain(|_, id| id != &method_id);
            if map.is_empty() {
                self.map.remove(&path_id);
            }
        }
    }
}
