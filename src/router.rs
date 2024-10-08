use crate::{
    constraints::Constraint,
    decode::percent_decode,
    errors::{ConstraintError, DeleteError, EncodingError, InsertError, SearchError},
    node::{search::Match, Children, Data, Kind, Node},
    parser::{Parser, Part},
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
                kind: Kind::Root,

                prefix: vec![],
                data: None,
                constraint: None,

                static_children: Children::default(),
                dynamic_children: Children::default(),
                wildcard_children: Children::default(),
                end_wildcard_children: Children::default(),

                quick_dynamic: false,
                needs_optimization: false,
            },
            constraints: HashMap::new(),
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
            return Err(EncodingError::EncodedRoute {
                input: route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            })?;
        }

        let route_arc = Arc::from(route);

        let mut parsed = Parser::new(route.as_bytes())?;
        for route in &parsed.routes {
            for part in &route.parts {
                if let Part::Dynamic {
                    constraint: Some(name),
                    ..
                }
                | Part::Wildcard {
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
        }

        if parsed.routes.len() > 1 {
            let value = Arc::new(value);
            for mut route in parsed.routes {
                let expanded = Arc::from(String::from_utf8_lossy(&route.raw));

                if let Err(err) = self.root.insert(
                    &mut route,
                    Data::Shared {
                        route: Arc::clone(&route_arc),
                        expanded,
                        value: Arc::clone(&value),
                    },
                ) {
                    // Attempt to clean up any prior inserts on failure.
                    // TODO: Consider adding tracing/log support?
                    // TODO: Consider returning a vec of errors?
                    drop(self.delete(&route_arc));
                    return Err(err);
                }
            }
        } else if let Some(route) = parsed.routes.first_mut() {
            self.root.insert(
                route,
                Data::Inline {
                    route: Arc::clone(&route_arc),
                    value,
                },
            )?;
        };

        self.root.optimize();
        Ok(())
    }

    /// Deletes a route from the router.
    ///
    /// The route provided must exactly match the route inserted.
    ///
    /// # Errors
    ///
    /// Returns a [`DeleteError`] if the route is invalid, cannot be deleted, or cannot be found.
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
        let decoded_route = percent_decode(route.as_bytes())?;
        if route.as_bytes() != decoded_route.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            })?;
        }

        let mut parsed = Parser::new(route.as_bytes())?;
        if parsed.routes.len() > 1 {
            let mut failure: Option<DeleteError> = None;
            for mut expanded_route in parsed.routes {
                // If a delete fails, keep trying the remaining paths, then return the first error.
                // TODO: Consider adding tracing/log support?
                // TODO: Consider returning a vec of errors?
                if let Err(err) = self.root.delete(&mut expanded_route, true) {
                    failure = Some(err);
                }
            }

            if let Some(err) = failure {
                return Err(err);
            }
        } else if let Some(route) = parsed.routes.first_mut() {
            self.root.delete(route, false)?;
        }

        self.root.optimize();
        Ok(())
    }

    /// Searches for a matching route in the router.
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

        let (route, expanded, data) = match &node.data {
            Some(Data::Inline { route, value }) => (Arc::clone(route), None, value),
            Some(Data::Shared {
                route,
                expanded,
                value,
            }) => (
                Arc::clone(route),
                Some(Arc::clone(expanded)),
                value.as_ref(),
            ),
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
