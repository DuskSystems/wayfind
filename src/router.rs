#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use crate::{
    chain::DataChain,
    errors::{DeleteError, InsertError, SearchError},
    MethodId, Request, Route,
};
use method::MethodRouter;
use path::{PathParameters, PathRouter};
use std::collections::BTreeMap;

pub mod method;
pub mod path;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'r, 'p, T> {
    pub data: &'r T,
    pub path: PathMatch<'r, 'p>,
    pub method: MethodMatch<'r>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathMatch<'r, 'p> {
    pub route: &'r str,
    pub expanded: Option<&'r str>,
    pub parameters: PathParameters<'r, 'p>,
}

impl<'r, 'p> From<path::PathMatch<'r, 'p>> for PathMatch<'r, 'p> {
    fn from(value: path::PathMatch<'r, 'p>) -> Self {
        Self {
            route: value.route,
            expanded: value.expanded,
            parameters: value.parameters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct MethodMatch<'r> {
    pub method: Option<&'r str>,
}

impl<'r> From<method::MethodMatch<'r>> for MethodMatch<'r> {
    fn from(value: method::MethodMatch<'r>) -> Self {
        Self {
            method: value.method,
        }
    }
}

#[derive(Clone)]
pub struct Router<'r, T> {
    pub path: PathRouter<'r>,
    pub method: MethodRouter,
    data: BTreeMap<DataChain, T>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: PathRouter::new(),
            method: MethodRouter::new(),
            data: BTreeMap::default(),
        }
    }

    pub fn insert(&mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        let path_id = self.path.insert(route.route)?;

        let method_id = if let Some(methods) = route.methods.as_ref() {
            self.method.insert(path_id, methods)?
        } else {
            MethodId(None)
        };

        let chain = DataChain {
            path: path_id,
            method: method_id,
        };

        if self.data.contains_key(&chain) {
            return Err(InsertError::Conflict { chain });
        }

        self.data.insert(chain, value);
        Ok(())
    }

    pub fn delete(&mut self, route: &Route<'r>) -> Result<T, DeleteError> {
        let Some(path_id) = self.path.find(route.route)? else {
            return Err(DeleteError::NotFound);
        };

        let method_id = if let Some(methods) = route.methods.as_ref() {
            self.method.find(path_id, methods)?
        } else {
            MethodId(None)
        };

        let chain = DataChain {
            path: path_id,
            method: method_id,
        };

        if !self.data.contains_key(&chain) {
            return Err(DeleteError::NotFound);
        }

        let path_count = self
            .data
            .keys()
            .filter(|existing| existing.path == path_id)
            .count();

        let method_count = if route.methods.is_some() {
            self.data
                .keys()
                .filter(|existing| existing.method == method_id)
                .count()
        } else {
            0
        };

        let data = self.data.remove(&chain).ok_or(DeleteError::NotFound)?;

        if path_count == 1 {
            self.path.delete(route.route);
        }

        if route.methods.is_some() && method_count == 1 {
            self.method.delete(path_id, method_id);
        }

        Ok(data)
    }

    pub fn search<'p>(
        &'r self,
        request: &'p Request<'p>,
    ) -> Result<Option<Match<'r, 'p, T>>, SearchError>
    where
        'p: 'r,
    {
        let Some(path) = self.path.search(request.path())? else {
            return Ok(None);
        };

        let path_id = path.id;

        let method = request
            .method()
            .map_or(Ok(None), |method| self.method.search(path_id, method));

        let method_id = method.as_ref().map_or_else(
            |_| MethodId(None),
            |method| method.as_ref().map_or(MethodId(None), |m| m.id),
        );

        let chain = DataChain {
            path: path_id,
            method: method_id,
        };

        let Some(data) = self.data.get(&chain) else {
            if let Err(err) = method {
                return Err(SearchError::Method(err));
            }

            return Ok(None);
        };

        let path = path.into();
        let method = method.map_or(MethodMatch::default(), |method| {
            method.map_or_else(MethodMatch::default, Into::into)
        });

        Ok(Some(Match { data, path, method }))
    }
}

impl<T> std::fmt::Display for Router<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n=== Path")?;
        let path = self.path.to_string();
        if path.is_empty() {
            write!(f, "\nEmpty")?;
        } else {
            write!(f, "\n{path}")?;
        }

        write!(f, "\n=== Method")?;
        let method = self.method.to_string();
        if method.is_empty() {
            write!(f, "\nEmpty")?;
        } else {
            write!(f, "\n{method}")?;
        }

        write!(f, "\n=== Chains")?;
        if self.data.is_empty() {
            write!(f, "\nEmpty")?;
        } else {
            for chain in self.data.keys() {
                write!(f, "\n{chain}")?;
            }
        }

        Ok(())
    }
}
