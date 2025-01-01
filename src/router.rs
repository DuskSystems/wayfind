#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use crate::{
    chain::DataChain,
    errors::{DeleteError, InsertError, SearchError},
    AuthorityId, MethodId, Request, Route,
};
use std::{collections::BTreeMap, sync::Arc};
use wayfind_authority::{AuthorityParameters, AuthorityRouter};
use wayfind_method::MethodRouter;
use wayfind_path::{PathParameters, PathRouter};

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'r, 'p, T> {
    pub data: &'r T,
    pub authority: AuthorityMatch<'r, 'p>,
    pub path: PathMatch<'r, 'p>,
    pub method: MethodMatch<'r>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct AuthorityMatch<'r, 'p> {
    pub authority: Option<Arc<str>>,
    pub parameters: AuthorityParameters<'r, 'p>,
}

impl<'r, 'p> From<wayfind_authority::AuthorityMatch<'r, 'p>> for AuthorityMatch<'r, 'p> {
    fn from(value: wayfind_authority::AuthorityMatch<'r, 'p>) -> Self {
        Self {
            authority: Some(value.authority),
            parameters: value.parameters,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathMatch<'r, 'p> {
    pub route: Arc<str>,
    pub expanded: Option<Arc<str>>,
    pub parameters: PathParameters<'r, 'p>,
}

impl<'r, 'p> From<wayfind_path::PathMatch<'r, 'p>> for PathMatch<'r, 'p> {
    fn from(value: wayfind_path::PathMatch<'r, 'p>) -> Self {
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

impl<'r> From<wayfind_method::MethodMatch<'r>> for MethodMatch<'r> {
    fn from(value: wayfind_method::MethodMatch<'r>) -> Self {
        Self {
            method: value.method,
        }
    }
}

#[derive(Clone)]
pub struct Router<T> {
    pub authority: AuthorityRouter,
    pub path: PathRouter,
    pub method: MethodRouter,
    data: BTreeMap<DataChain, T>,
}

impl<T> Router<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            authority: AuthorityRouter::new(),
            path: PathRouter::new(),
            method: MethodRouter::new(),
            data: BTreeMap::default(),
        }
    }

    pub fn insert<'r>(&'r mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        let authority_id = if let Some(authority) = &route.authority {
            self.authority.insert(None, authority)?
        } else {
            AuthorityId(None)
        };

        let path_id = self.path.insert(authority_id.0, &route.route)?;

        let method_id = if let Some(methods) = route.methods.as_ref() {
            self.method.insert(path_id.0, methods)?
        } else {
            MethodId(None)
        };

        let chain = DataChain {
            authority: authority_id,
            path: path_id,
            method: method_id,
        };

        if self.data.contains_key(&chain) {
            return Err(InsertError::Conflict { chain });
        }

        self.data.insert(chain, value);
        Ok(())
    }

    pub fn delete(&mut self, route: &Route<'_>) -> Result<T, DeleteError> {
        let authority_id = if let Some(authority) = route.authority.as_ref() {
            self.authority
                .find(None, authority)?
                .ok_or(DeleteError::NotFound)?
        } else {
            AuthorityId(None)
        };

        let Some(path_id) = self.path.find(authority_id.0, &route.route)? else {
            return Err(DeleteError::NotFound);
        };

        let method_id = if let Some(methods) = route.methods.as_ref() {
            self.method.find(path_id.0, methods)?
        } else {
            MethodId(None)
        };

        let chain = DataChain {
            authority: authority_id,
            path: path_id,
            method: method_id,
        };

        if !self.data.contains_key(&chain) {
            return Err(DeleteError::NotFound);
        }

        let authority_count = if route.authority.is_some() {
            self.data
                .keys()
                .filter(|existing| existing.authority == authority_id)
                .count()
        } else {
            0
        };

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

        if authority_count == 1 {
            if let Some(authority) = &route.authority {
                self.authority.delete(None, authority);
            };
        }

        if path_count == 1 {
            self.path.delete(authority_id.0, &route.route);
        }

        if route.methods.is_some() && method_count == 1 {
            self.method.delete(path_id.0, method_id);
        }

        Ok(data)
    }

    pub fn search<'r, 'p>(
        &'r self,
        request: &'p Request<'p>,
    ) -> Result<Option<Match<'r, 'p, T>>, SearchError>
    where
        'p: 'r,
    {
        let authority = request.authority().map_or_else(
            || Ok(None),
            |authority| self.authority.search(None, authority.as_bytes()),
        );

        let authority_id = authority.as_ref().map_or_else(
            |_| AuthorityId(None),
            |authority| authority.as_ref().map_or(AuthorityId(None), |a| a.id),
        );

        let Some(path) = self.path.search(authority_id.0, request.path())? else {
            return Ok(None);
        };

        let path_id = path.id;

        let method = request
            .method()
            .map_or(Ok(None), |method| self.method.search(path_id.0, method));

        let method_id = method.as_ref().map_or_else(
            |_| MethodId(None),
            |method| method.as_ref().map_or(MethodId(None), |m| m.id),
        );

        let chain = DataChain {
            authority: authority_id,
            path: path_id,
            method: method_id,
        };

        let Some(data) = self.data.get(&chain) else {
            authority?;
            method?;
            return Ok(None);
        };

        let authority = authority.map_or(AuthorityMatch::default(), |authority| {
            authority.map_or_else(AuthorityMatch::default, Into::into)
        });

        let path = path.into();

        let method = method.map_or(MethodMatch::default(), |method| {
            method.map_or_else(MethodMatch::default, Into::into)
        });

        Ok(Some(Match {
            data,
            authority,
            path,
            method,
        }))
    }
}

impl<T> std::fmt::Display for Router<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n=== Authority")?;
        let authority = self.authority.to_string();
        if authority.is_empty() {
            write!(f, "\nEmpty")?;
        } else {
            write!(f, "\n{authority}")?;
        }

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
