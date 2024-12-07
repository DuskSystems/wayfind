use crate::{
    errors::{DeleteError, InsertError, SearchError},
    id::RouteId,
    map::RouteMap,
    Request, Route,
};
use alloc::string::ToString;
use path::{PathParameters, PathRouter};

pub mod path;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'r, 'p, T> {
    pub data: &'r T,
    pub path: PathMatch<'r, 'p>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathMatch<'r, 'p> {
    pub route: &'r str,
    pub expanded: Option<&'r str>,
    pub parameters: PathParameters<'r, 'p>,
}

#[derive(Clone)]
pub struct Router<'r, T> {
    pub path: PathRouter<'r>,
    data: RouteMap<T>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: PathRouter::new(),
            data: RouteMap::default(),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn insert(&mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        let id = RouteId::new();

        self.path.insert(route, id)?;
        self.data.insert(id, value);

        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn delete(&mut self, route: &Route<'r>) -> Result<(), DeleteError> {
        let path_data = self.path.delete(route)?;

        let path_id = path_data.id;
        self.data.remove(&path_id);

        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn search<'p>(
        &'r self,
        request: &'p Request<'p>,
    ) -> Result<Option<Match<'r, 'p, T>>, SearchError> {
        let Some(search) = self.path.search(request, &self.data)? else {
            return Ok(None);
        };

        Ok(Some(Match {
            data: search.data,
            path: PathMatch {
                route: search.route,
                expanded: search.expanded,
                parameters: search.parameters,
            },
        }))
    }
}

impl<T> core::fmt::Display for Router<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "=== Path")?;
        let path = self.path.to_string();
        if !path.is_empty() {
            write!(f, "\n{path}")?;
        }

        Ok(())
    }
}
