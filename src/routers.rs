use crate::{
    chain::DataChain,
    errors::{DeleteError, InsertError, SearchError},
    Request, Route,
};
use path::{PathParameters, PathRouter};
use std::collections::BTreeMap;

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
    data: BTreeMap<DataChain, T>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: PathRouter::new(),
            data: BTreeMap::default(),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn insert(&mut self, route: &Route<'r>, value: T) -> Result<(), InsertError> {
        let path_id = self.path.insert(route.route)?;

        let chain = DataChain { path: path_id };
        self.data.insert(chain, value);

        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn delete(&mut self, route: &Route<'r>) -> Result<(), DeleteError> {
        let path_data = self.path.delete(route.route)?;

        let chain = DataChain { path: path_data.id };
        self.data.remove(&chain);

        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn search<'p>(
        &'r self,
        request: &'p Request<'p>,
    ) -> Result<Option<Match<'r, 'p, T>>, SearchError> {
        let Some(search) = self.path.search(request.path.as_ref())? else {
            return Ok(None);
        };

        let chain = DataChain { path: search.id };
        let Some(data) = self.data.get(&chain) else {
            return Ok(None);
        };

        Ok(Some(Match {
            data,
            path: PathMatch {
                route: search.route,
                expanded: search.expanded,
                parameters: search.parameters,
            },
        }))
    }
}

impl<T> std::fmt::Display for Router<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "=== Path")?;
        let path = self.path.to_string();
        if path.is_empty() {
            write!(f, "\nEmpty")?;
        } else {
            write!(f, "\n{path}")?;
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
