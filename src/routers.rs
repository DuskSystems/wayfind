use crate::{
    errors::{DeleteError, InsertError, SearchError},
    id::RouteId,
    storage::{Storage, StorageKind},
    Request, Route,
};
use hashbrown::HashMap;
use path::{node::Data, PathMatch, PathRouter};

pub mod path;

pub type Match<'r, 'p, T> = PathMatch<'r, 'p, T>;

#[derive(Clone)]
pub struct Router<'r, T> {
    pub path: PathRouter<'r, T>,

    /// Stored data.
    data: HashMap<RouteId, T>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: PathRouter::new(),
            data: HashMap::new(),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn insert(&mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        let value = match route.storage {
            StorageKind::Inline => Some(value),
            StorageKind::Router(id) => {
                self.data.insert(id, value);
                None
            }
        };

        self.path.insert(route, value)?;

        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn delete(&mut self, route: &Route<'r>) -> Result<(), DeleteError> {
        let data = self.path.delete(route)?;

        match &data {
            Data::Inline { storage, .. } => match storage {
                Storage::Inline(_) => (),
                Storage::Router(id) => {
                    self.data.remove(id);
                }
            },
            Data::Shared { storage, .. } => match storage.as_ref() {
                Storage::Inline(_) => (),
                Storage::Router(id) => {
                    self.data.remove(id);
                }
            },
        };

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
