use crate::{
    constraints::Constraint,
    decode::percent_decode,
    errors::{ConstraintError, DeleteError, EncodingError, InsertError, SearchError},
    node::{Children, Data, Kind, Node},
    parser::{Parser, Part},
    Routable,
};
use http::Request;
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

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct Match<'router, T> {
    /// The matching route.
    pub route: Arc<str>,

    /// The expanded route, if applicable.
    pub expanded: Option<Arc<str>>,

    /// A reference to the matching route data.
    pub data: &'router T,

    /// Key-value pairs of parameters, extracted from the route.
    pub parameters: Vec<Parameter>,
}

/// A key-value parameter pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub key: Arc<str>,
    pub value: Arc<str>,
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
                dynamic_children_shortcut: false,
                wildcard_children: Children::default(),
                wildcard_children_shortcut: false,
                end_wildcard_children: Children::default(),

                priority: 0,
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

    /// Inserts a new routable with an associated value into the router.
    ///
    /// The route should not contain any percent-encoded characters.
    ///
    /// # Errors
    ///
    /// Returns an [`InsertError`] if the routable is invalid or uses unknown constraints.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router, Routable};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    ///
    /// let route = Routable::builder()
    ///     .route("/hello/{world}")
    ///     .build()
    ///     .unwrap();
    ///
    /// router.insert(route, 2).unwrap();
    /// ```
    pub fn insert<'a>(
        &mut self,
        routable: impl Into<Routable<'a>>,
        value: T,
    ) -> Result<(), InsertError> {
        let routable = routable.into();

        let decoded = percent_decode(routable.route.as_bytes())?;
        if routable.route.as_bytes() != decoded.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: routable.route.to_string(),
                decoded: String::from_utf8_lossy(&decoded).to_string(),
            })?;
        }

        let route_arc = Arc::from(routable.route);

        let mut parsed = Parser::new(routable.route.as_bytes())?;
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
                    // TODO: Consider returning a vec of errors?
                    drop(self.delete(&*route_arc));
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
    /// use wayfind::{Constraint, Router, Routable};
    ///
    /// let mut router: Router<usize> = Router::new();
    ///
    /// let route = Routable::builder()
    ///     .route("/hello")
    ///     .build()
    ///     .unwrap();
    ///
    /// router.insert(route.clone(), 1).unwrap();
    /// router.delete(route).unwrap();
    /// ```
    pub fn delete<'a>(&mut self, routable: impl Into<Routable<'a>>) -> Result<(), DeleteError> {
        let routable = routable.into();

        let decoded = percent_decode(routable.route.as_bytes())?;
        if routable.route.as_bytes() != decoded.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: routable.route.to_string(),
                decoded: String::from_utf8_lossy(&decoded).to_string(),
            })?;
        }

        let mut parsed = Parser::new(routable.route.as_bytes())?;
        if parsed.routes.len() > 1 {
            let mut failure: Option<DeleteError> = None;
            for mut expanded_route in parsed.routes {
                // If a delete fails, keep trying the remaining paths, then return the first error.
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

    /// Searches for a matching routable in the router.
    ///
    /// # Errors
    ///
    /// Returns a [`SearchError`] if the search resulted in invalid parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    ///
    /// let search = router.search("/hello").unwrap();
    /// ```
    pub fn search<'router>(
        &'router self,
        path: &str,
    ) -> Result<Option<Match<'router, T>>, SearchError> {
        let decoded = percent_decode(path.as_bytes())?;

        let mut parameters = vec![];
        let Some(node) = self
            .root
            .search(&decoded, &mut parameters, &self.constraints)?
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

        let parameters: Result<Vec<Parameter>, SearchError> = parameters
            .iter_mut()
            .map(|parameter| {
                let key =
                    std::str::from_utf8(&parameter.key).map_err(|_| SearchError::Utf8Error {
                        key: String::from_utf8_lossy(&parameter.key).to_string(),
                        value: String::from_utf8_lossy(&parameter.value).to_string(),
                    })?;

                let value =
                    std::str::from_utf8(&parameter.value).map_err(|_| SearchError::Utf8Error {
                        key: String::from_utf8_lossy(&parameter.key).to_string(),
                        value: String::from_utf8_lossy(&parameter.value).to_string(),
                    })?;

                Ok(Parameter {
                    key: key.into(),
                    value: value.into(),
                })
            })
            .collect();

        let parameters = parameters?;

        Ok(Some(Match {
            route,
            expanded,
            data,
            parameters,
        }))
    }

    /// Searches for a matching routable in the router using a HTTP request.
    ///
    /// # Errors
    ///
    /// Returns a [`SearchError`] if the search resulted in invalid parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::Router;
    /// use http::Request;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    ///
    /// let request = Request::builder()
    ///     .uri("/hello")
    ///     .body(())
    ///     .unwrap();
    ///
    /// let search = router.search_request(&request).unwrap();
    /// ```
    pub fn search_request<'router, B>(
        &'router self,
        request: &'router Request<B>,
    ) -> Result<Option<Match<'router, T>>, SearchError> {
        let path = request.uri().path();
        self.search(path)
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
