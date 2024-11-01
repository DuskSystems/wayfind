use crate::{
    decode::percent_decode,
    errors::{DeleteError, EncodingError, InsertError, SearchError},
    node::{Children, Data, Kind, Node},
    parser::Parser,
    path::Path,
};
use smallvec::{smallvec, SmallVec};
use std::fmt::Display;

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct Match<'router, 'path, T> {
    /// The matching route.
    pub route: &'router str,

    /// The expanded route, if applicable.
    pub expanded: Option<&'router str>,

    /// A reference to the matching route data.
    pub data: &'router T,

    /// Key-value pairs of parameters, extracted from the route.
    pub parameters: Parameters<'router, 'path>,
}

/// All the parameter pairs of a given match.
pub type Parameters<'router, 'path> = SmallVec<[Parameter<'router, 'path>; 4]>;

/// A key-value parameter pair.
///
/// The key of the parameter is tied to the lifetime of the router, since it is a ref to the prefix of a given node.
/// Meanwhile, the value is extracted from the path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'router, 'path> {
    pub key: &'router str,
    pub value: &'path str,
}

/// The [`wayfind`](crate) router.
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<'router, T> {
    /// The root node of the tree.
    root: Node<'router, T>,
}

impl<'router, T> Router<'router, T> {
    /// Creates a new Router.
    ///
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node {
                kind: Kind::Root,

                prefix: &[],
                data: None,

                static_children: Children::default(),
                dynamic_children: Children::default(),
                dynamic_children_shortcut: false,
                wildcard_children: Children::default(),
                wildcard_children_shortcut: false,
                end_wildcard_children: Children::default(),

                priority: 0,
                needs_optimization: false,
            },
        }
    }

    /// Inserts a new routable with an associated value into the router.
    ///
    /// The route should not contain any percent-encoded characters.
    ///
    /// # Errors
    ///
    /// Returns an [`InsertError`] if the routable is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// ```
    pub fn insert(&mut self, routable: &'router str, value: T) -> Result<(), InsertError> {
        let decoded_route = percent_decode(routable.as_bytes())?;
        if routable.as_bytes() != decoded_route.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: routable.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            })?;
        }

        let mut parsed = Parser::new(routable.as_bytes())?;
        self.root.insert(
            &mut parsed.route,
            Data {
                route: routable,
                value,
            },
        )?;

        self.root.optimize();
        Ok(())
    }

    /// Deletes a routable from the router.
    ///
    /// The routable provided must exactly match the routable inserted.
    ///
    /// # Errors
    ///
    /// Returns a [`DeleteError`] if the routable is invalid, cannot be deleted, or cannot be found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// router.delete("/hello").unwrap();
    /// ```
    pub fn delete(&mut self, routable: &'router str) -> Result<(), DeleteError> {
        let decoded_route = percent_decode(routable.as_bytes())?;
        if routable.as_bytes() != decoded_route.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: routable.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            })?;
        }

        let mut parsed = Parser::new(routable.as_bytes())?;
        self.root.delete(&mut parsed.route)?;

        self.root.optimize();
        Ok(())
    }

    /// Searches for a matching routable in the router.
    ///
    /// # Errors
    ///
    /// Returns a [`SearchError`] if the search resulted in invalid parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Path, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    ///
    /// let path = Path::new("/hello").unwrap();
    /// let search = router.search(&path).unwrap();
    /// ```
    pub fn search<'path>(
        &'router self,
        path: &'path Path<'_>,
    ) -> Result<Option<Match<'router, 'path, T>>, SearchError> {
        let mut parameters = smallvec![];
        let Some(node) = self.root.search(path.decoded_bytes(), &mut parameters)? else {
            return Ok(None);
        };

        let (route, expanded, data) = match &node.data {
            Some(Data { route, value }) => (route, None, value),
            None => return Ok(None),
        };

        Ok(Some(Match {
            route,
            expanded,
            data,
            parameters,
        }))
    }
}

impl<'router, T> Default for Router<'router, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'router, T> Display for Router<'router, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
