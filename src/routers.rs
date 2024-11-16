use crate::{
    errors::{DeleteError, InsertError, SearchError},
    id::RouteId,
    map::RouteMap,
    Request, Route,
};
use path::{PathMatch, PathRouter};

pub mod path;

pub type Match<'r, 'p, T> = PathMatch<'r, 'p, T>;

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
    ) -> Result<Option<PathMatch<'r, 'p, T>>, SearchError> {
        let search = self.path.search(request, &self.data)?;
        Ok(search)
    }
}

impl<'r, T> Default for Router<'r, T> {
    fn default() -> Self {
        Self::new()
    }
}
