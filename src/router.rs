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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(constraint = ?C::NAME)))]
    pub fn constraint<C: Constraint>(&mut self) -> Result<(), ConstraintError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Adding constraint");

        match self.constraints.entry(C::NAME.as_bytes().to_vec()) {
            Entry::Vacant(entry) => {
                entry.insert(StoredConstraint {
                    type_name: type_name::<C>(),
                    check: C::check,
                });

                #[cfg(feature = "tracing")]
                tracing::debug!("Constraint added successfully");

                Ok(())
            }
            Entry::Occupied(entry) => {
                let error = Err(ConstraintError::DuplicateName {
                    name: C::NAME,
                    existing_type: entry.get().type_name,
                    new_type: type_name::<C>(),
                });

                #[cfg(feature = "tracing")]
                tracing::error!(
                    error = ?error,
                    "Failed to add constraint"
                );

                error
            }
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(route = ?route)))]
    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Inserting route");

        let decoded_route = percent_decode(route.as_bytes())?;
        if route.as_bytes() != decoded_route.as_ref() {
            let error = Err(EncodingError::EncodedRoute {
                input: route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            });

            #[cfg(feature = "tracing")]
            tracing::error!(
                error = ?error,
                "Failed to insert route: encoded route"
            );

            return error?;
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
                        let error = Err(InsertError::UnknownConstraint {
                            constraint: String::from_utf8_lossy(name).to_string(),
                        });

                        #[cfg(feature = "tracing")]
                        tracing::error!(
                            error = ?error,
                            "Failed to insert route: unknown constraint"
                        );

                        return error;
                    }
                }
            }
        }

        if parsed.is_expanded {
            let value = Arc::new(value);
            for mut route in parsed.routes {
                let expanded = Arc::from(String::from_utf8_lossy(&route.raw));

                #[cfg(feature = "tracing")]
                tracing::debug!(
                    insert = ?expanded,
                    "Inserting parsed, expanded route"
                );

                if let Err(error) = self.root.insert(
                    &mut route,
                    Data::Shared {
                        route: Arc::clone(&route_arc),
                        expanded,
                        value: Arc::clone(&value),
                    },
                ) {
                    #[cfg(feature = "tracing")]
                    tracing::error!(
                        error = ?error,
                        "Failed to insert expanded route"
                    );

                    // Attempt to clean up any invalid state.
                    drop(self.delete(&route_arc));

                    return Err(error);
                }
            }
        } else {
            let route = &mut parsed.routes[0];

            #[cfg(feature = "tracing")]
            tracing::debug!(
                insert = ?route,
                "Inserting parsed route"
            );

            let result = self.root.insert(
                route,
                Data::Inline {
                    route: Arc::clone(&route_arc),
                    value,
                },
            );

            #[cfg(feature = "tracing")]
            if let Err(error) = result {
                tracing::error!(
                    error = ?error,
                    "Failed to insert route"
                );

                return Err(error);
            }

            result?;
        };

        self.root.optimize();

        #[cfg(feature = "tracing")]
        tracing::debug!("Route inserted successfully");

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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(route = ?route)))]
    pub fn delete(&mut self, route: &str) -> Result<(), DeleteError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Deleting route");

        let decoded_route = percent_decode(route.as_bytes())?;
        if route.as_bytes() != decoded_route.as_ref() {
            let error = Err(EncodingError::EncodedRoute {
                input: route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            });

            #[cfg(feature = "tracing")]
            tracing::error!(
                error = ?error,
                "Failed to delete route: encoded route"
            );

            return error?;
        }

        let parsed = Parser::new(route.as_bytes())?;
        for mut route in parsed.routes {
            if let Err(error) = self.root.delete(&mut route, parsed.is_expanded) {
                #[cfg(feature = "tracing")]
                tracing::error!(
                    error = ?error,
                    "Failed to delete route"
                );

                // Attempt to clean up any invalid state.
                self.root.optimize();

                return Err(error);
            }
        }

        self.root.optimize();

        #[cfg(feature = "tracing")]
        tracing::debug!("Route deleted successfully");

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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(path = ?path)))]
    pub fn search<'router, 'path>(
        &'router self,
        path: &'path Path<'_>,
    ) -> Result<Option<Match<'router, 'path, T>>, SearchError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Searching for route");

        let mut parameters = vec![];
        let Some(node) =
            self.root
                .search(path.decoded_bytes(), &mut parameters, &self.constraints)?
        else {
            #[cfg(feature = "tracing")]
            tracing::debug!("No matching route found");

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
            None => {
                #[cfg(feature = "tracing")]
                tracing::debug!("Matched node with no data");

                return Ok(None);
            }
        };

        #[cfg(feature = "tracing")]
        tracing::debug!(
            route = ?route,
            expanded = ?expanded,
            parameters = ?parameters,
            "Found matching route"
        );

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
