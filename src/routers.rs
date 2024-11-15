use crate::{
    errors::{DeleteError, InsertError, SearchError},
    Request, Route,
};
use path::{PathMatch, PathRouter};

pub mod path;

pub type Match<'r, 'p, T> = PathMatch<'r, 'p, T>;

#[derive(Clone)]
pub struct Router<'r, T> {
    pub path: PathRouter<'r, T>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: PathRouter::new(),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn insert(&mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        self.path.insert(route, value)?;
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn delete(&mut self, route: &Route<'r>) -> Result<(), DeleteError> {
        self.path.delete(route)?;
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn search<'p>(
        &'r self,
        request: &'p Request<'p>,
    ) -> Result<Option<PathMatch<'r, 'p, T>>, SearchError> {
        let search = self.path.search(request)?;
        Ok(search)
    }
}

impl<'r, T> Default for Router<'r, T> {
    fn default() -> Self {
        Self::new()
    }
}
