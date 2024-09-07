use crate::{
    constraints::Constraint,
    decode::percent_decode,
    errors::{ConstraintError, DeleteError, InsertError, SearchError},
    node::{search::Match, Node, NodeData, NodeKind},
    parser::{ParsedRoute, RoutePart},
    path::Path,
};
use std::{
    any::type_name,
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

/// A constraint with its type name.
#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

/// The [`wayfind`](crate) router.
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<T> {
    /// The root node of the tree.
    root: Node<T>,

    /// A map of constraint names to [`StoredConstraint`].
    constraints: HashMap<Vec<u8>, StoredConstraint>,

    /// Shared route data.
    /// Only used when multiple nodes require the same data storage.
    data: HashMap<Arc<str>, T>,
}

impl<T> Router<T> {
    /// Creates a new Router with default constraints.
    ///
    /// # Panics
    ///
    /// Can only panic if the default constraint registrations fail, which should never happen.
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            root: Node {
                kind: NodeKind::Root,

                prefix: vec![],
                data: None,
                constraint: None,

                static_children: vec![],
                dynamic_children: vec![],
                wildcard_children: vec![],
                end_wildcard_children: vec![],

                quick_dynamic: false,
            },
            constraints: HashMap::new(),
            data: HashMap::new(),
        };

        router.constraint::<u8>().unwrap();
        router.constraint::<u16>().unwrap();
        router.constraint::<u32>().unwrap();
        router.constraint::<u64>().unwrap();
        router.constraint::<u128>().unwrap();
        router.constraint::<usize>().unwrap();
        router.constraint::<i8>().unwrap();
        router.constraint::<i16>().unwrap();
        router.constraint::<i32>().unwrap();
        router.constraint::<i64>().unwrap();
        router.constraint::<i128>().unwrap();
        router.constraint::<isize>().unwrap();
        router.constraint::<f32>().unwrap();
        router.constraint::<f64>().unwrap();
        router.constraint::<bool>().unwrap();
        router.constraint::<Ipv4Addr>().unwrap();
        router.constraint::<Ipv6Addr>().unwrap();

        router
    }

    /// Registers a new constraint to the router.
    ///
    /// # Errors
    ///
    /// Returns a [`ConstraintError`] if the constraint could not be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// struct HelloConstraint;
    /// impl Constraint for HelloConstraint {
    ///     const NAME: &'static str = "hello";
    ///
    ///     fn check(segment: &str) -> bool {
    ///         segment == "hello"
    ///     }
    /// }
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.constraint::<HelloConstraint>().unwrap();
    /// ```
    pub fn constraint<C: Constraint>(&mut self) -> Result<(), ConstraintError> {
        match self.constraints.entry(C::NAME.as_bytes().to_vec()) {
            Entry::Vacant(entry) => {
                entry.insert(StoredConstraint {
                    type_name: type_name::<C>(),
                    check: C::check,
                });

                Ok(())
            }
            Entry::Occupied(entry) => Err(ConstraintError::DuplicateName {
                name: C::NAME,
                existing_type: entry.get().type_name,
                new_type: type_name::<C>(),
            }),
        }
    }

    /// Inserts a new route with an associated value into the router.
    ///
    /// The route should not contain any percent-encoded characters.
    ///
    /// # Errors
    ///
    /// Returns an [`InsertError`] if the route is invalid or uses unknown constraints.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// router.insert("/hello/{world}", 2).unwrap();
    /// ```
    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
        let decoded_route = percent_decode(route.as_bytes())?;
        if route.as_bytes() != decoded_route.as_ref() {
            return Err(InsertError::EncodedRoute {
                input: route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            });
        }

        let route_arc = Arc::from(route);

        let mut parts = ParsedRoute::new(route.as_bytes())?;
        for part in &parts {
            if let RoutePart::Dynamic {
                constraint: Some(name),
                ..
            }
            | RoutePart::Wildcard {
                constraint: Some(name),
                ..
            } = part
            {
                if !self.constraints.contains_key(name) {
                    return Err(InsertError::UnknownConstraint {
                        constraint: String::from_utf8_lossy(name).to_string(),
                    });
                }
            }
        }

        // TODO: Using Parts, determine whether we need to store inline or not.
        let node_data = NodeData::Inline {
            route: Arc::clone(&route_arc),
            value,
        };

        self.root.insert(&mut parts, node_data)
    }

    /// Deletes a route from the router.
    ///
    /// The route provided must exactly match the route inserted.
    ///
    /// # Errors
    ///
    /// Returns a [`DeleteError`] if the route cannot be deleted, or cannot be found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// router.delete("/hello").unwrap();
    /// ```
    pub fn delete(&mut self, route: &str) -> Result<(), DeleteError> {
        let mut parts = ParsedRoute::new(route.as_bytes())?;
        self.root.delete(&mut parts)
    }

    /// Searches for a matching route in the router.
    ///
    /// Returns a [`Match`] if a matching route is found, or [`None`] otherwise.
    ///
    /// # Errors
    ///
    /// Returns a [`SearchError`] if the search resulted in invalid parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Path, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    ///
    /// let path = Path::new("/hello").unwrap();
    /// let search = router.search(&path).unwrap();
    /// ```
    pub fn search<'router, 'path>(
        &'router self,
        path: &'path Path<'_>,
    ) -> Result<Option<Match<'router, 'path, T>>, SearchError> {
        let mut parameters = vec![];
        let Some(node) =
            self.root
                .search(path.decoded_bytes(), &mut parameters, &self.constraints)?
        else {
            return Ok(None);
        };

        let (route, data) = match &node.data {
            Some(NodeData::Inline { route, value }) => (Arc::clone(route), value),
            Some(NodeData::Reference(key)) => {
                let Some(data) = self.data.get(key) else {
                    return Ok(None);
                };

                (Arc::clone(key), data)
            }
            None => return Ok(None),
        };

        Ok(Some(Match {
            route,
            data,
            parameters,
        }))
    }
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Display for Router<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
