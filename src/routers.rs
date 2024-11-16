use crate::{
    errors::{DeleteError, InsertError, SearchError},
    map::{ChainMap, DataChain},
    Request, Route,
};
use method::MethodRouter;
use path::{errors::PathInsertError, PathParameters, PathRouter};

pub mod authority;
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
    data: ChainMap<T>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: PathRouter::new(),
            method: MethodRouter::new(),
            data: ChainMap::default(),
        }
    }

    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    pub fn insert(&mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        let (path_id, path_err) = match self.path.insert(route.route) {
            Ok(id) => (id, None),
            Err(err) => match err {
                PathInsertError::DuplicateRoute { id, .. } => (id, Some(err)),
                _ => return Err(err)?,
            },
        };

        let method_id = if let Some(methods) = route.methods.as_ref() {
            self.method.insert(route.route, methods)?
        } else {
            self.method.id.none()
        };

        // FIXME: How to handle duplicates here?
        let chain = DataChain::new(path_id, method_id);

        if self.data.contains_key(&chain) {
            if let Some(err) = path_err {
                return Err(InsertError::Path(err));
            }

            panic!("Unexpected conflict.");
        }

        self.data.insert(chain, value);
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn delete(&mut self, route: &Route<'r>) -> Result<(), DeleteError> {
        // FIXME: We really need to perform a full 'find', prior to deleting.
        // This will tell us whether we need to call the underlying delete methods or not.
        // So we'll want a way to find the path ID.
        // If it doesn't exist, return not found.
        // If it does, check for methods.
        // Perform a method 'find'.
        // And so on.
        // We'll want some sort of 'conflict' detector.
        // A way to see if a conflict exists for path, for method.
        // To determine whether we need to call delete.
        // Slower - but deletes are rare.
        let path_id = self.path.delete(route.route)?;

        let method_id = if let Some(methods) = route.methods.as_ref() {
            self.method.delete(route.route, methods)?
        } else {
            self.method.id.none()
        };

        let chain = DataChain::new(path_id, method_id);
        self.data.remove(&chain);

        Ok(())
    }

    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
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

        let method = if let Some(method) = request.method() {
            self.method.search(path.route, method)
        } else {
            Ok(None)
        };

        let method_id = method.as_ref().map_or_else(
            |_| self.method.id.none(),
            |method| method.as_ref().map_or(self.method.id.none(), |m| m.id),
        );

        let chain = DataChain::new(path_id, method_id);
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
        write!(f, "=== Path")?;
        let path = self.path.to_string();
        if !path.is_empty() {
            write!(f, "\n{path}")?;
        }

        write!(f, "\n=== Method")?;
        let method = self.method.to_string();
        if !method.is_empty() {
            write!(f, "\n{method}")?;
        }

        Ok(())
    }
}
