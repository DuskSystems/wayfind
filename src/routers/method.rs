use errors::{MethodDeleteError, MethodInsertError, MethodSearchError};
use std::collections::BTreeMap;

pub mod display;
pub mod errors;
pub mod id;

pub use id::MethodId;
use id::MethodIdGenerator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodMatch<'r> {
    pub id: MethodId,
    pub method: Option<&'r str>,
}

#[derive(Clone)]
pub struct MethodRouter {
    map: BTreeMap<String, BTreeMap<String, MethodId>>,
    pub(super) id: MethodIdGenerator,
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
        route: &str,
        methods: &Vec<&str>,
    ) -> Result<MethodId, MethodInsertError> {
        if methods.is_empty() {
            return Err(MethodInsertError::Empty);
        }

        let map = self.map.entry(route.to_owned()).or_default();

        for method in methods {
            if map.contains_key(*method) {
                return Err(MethodInsertError::Conflict {
                    route: route.to_owned(),
                    method: (*method).to_owned(),
                });
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
        path: &str,
        method: &'r str,
    ) -> Result<Option<MethodMatch<'r>>, MethodSearchError> {
        if let Some(map) = self.map.get(path) {
            if let Some(id) = map.get(method) {
                return Ok(Some(MethodMatch {
                    id: *id,
                    method: Some(method),
                }));
            }

            return Err(MethodSearchError::NotAllowed);
        }

        Ok(None)
    }

    pub fn delete(
        &mut self,
        route: &str,
        methods: &Vec<&str>,
    ) -> Result<MethodId, MethodDeleteError> {
        let map = self.map.get(route).ok_or(MethodDeleteError::NotFound)?;

        for method in methods {
            if !map.contains_key(*method) {
                return Err(MethodDeleteError::Mismatch);
            }
        }

        let first_id = map.get(methods[0]).unwrap();

        for method in methods {
            match map.get(*method) {
                Some(id) if id == first_id => continue,
                _ => return Err(MethodDeleteError::Mismatch),
            }
        }

        if map
            .iter()
            .any(|(method, id)| id == first_id && !methods.contains(&method.as_str()))
        {
            return Err(MethodDeleteError::Mismatch);
        }

        let id = *first_id;

        if let Some(map) = self.map.get_mut(route) {
            map.retain(|_, mid| mid != &id);
            if map.is_empty() {
                self.map.remove(route);
            }
        }

        Ok(id)
    }
}
