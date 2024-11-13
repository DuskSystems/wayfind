use crate::{
    constraints::Constraint,
    decode::percent_decode,
    errors::{ConstraintError, DeleteError, EncodingError, InsertError, SearchError},
    node::{Children, Data, Node},
    parser::{Parser, Part},
    path::Path,
    state::RootState,
    Routable,
};
use alloc::{
    fmt::Display,
    string::{String, ToString},
    sync::Arc,
};
use core::{
    any::type_name,
    net::{Ipv4Addr, Ipv6Addr},
};
use hashbrown::HashMap;
use smallvec::{smallvec, SmallVec};

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct Match<'r, 'p, T> {
    /// The matching route.
    pub route: &'r str,

    /// The expanded route, if applicable.
    pub expanded: Option<&'r str>,

    /// A reference to the matching route data.
    pub data: &'r T,

    /// Key-value pairs of parameters, extracted from the route.
    pub parameters: Parameters<'r, 'p>,
}

/// All the parameter pairs of a given match.
///
/// The key of the parameter is tied to the lifetime of the router, since it is a ref to the prefix of a given node.
/// Meanwhile, the value is extracted from the path.
pub type Parameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

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
pub struct Router<'r, T> {
    /// The root node of the tree.
    root: Node<'r, T, RootState>,

    /// A map of constraint names to [`StoredConstraint`].
    constraints: HashMap<&'r str, StoredConstraint>,
}

impl<'r, T> Router<'r, T> {
    /// Creates a new Router with default constraints.
    ///
    /// # Panics
    ///
    /// Can only panic if the default constraint registrations fail, which should never happen.
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            root: Node {
                state: RootState::new(),
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
            constraints: HashMap::default(),
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
        if let Some(existing) = self.constraints.get(&C::NAME) {
            return Err(ConstraintError::DuplicateName {
                name: C::NAME,
                existing_type: existing.type_name,
                new_type: type_name::<C>(),
            });
        }

        self.constraints.insert(
            C::NAME,
            StoredConstraint {
                type_name: type_name::<C>(),
                check: C::check,
            },
        );

        Ok(())
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
    pub fn insert(
        &mut self,
        routable: impl Into<Routable<'r>>,
        value: T,
    ) -> Result<(), InsertError> {
        let routable = routable.into();

        let decoded_route = percent_decode(routable.route.as_bytes())?;
        if routable.route.as_bytes() != decoded_route.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: routable.route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
            })?;
        }

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
                    if !self.constraints.contains_key(name.as_str()) {
                        return Err(InsertError::UnknownConstraint {
                            constraint: name.to_string(),
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
                        route: routable.route,
                        expanded,
                        value: Arc::clone(&value),
                    },
                ) {
                    // Attempt to clean up any prior inserts on failure.
                    // TODO: Consider returning a vec of errors?
                    drop(self.delete(routable.route));
                    return Err(err);
                }
            }
        } else if let Some(route) = parsed.routes.first_mut() {
            self.root.insert(
                route,
                Data::Inline {
                    route: routable.route,
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
    pub fn delete(&mut self, routable: impl Into<Routable<'r>>) -> Result<(), DeleteError> {
        let routable = routable.into();

        let decoded_route = percent_decode(routable.route.as_bytes())?;
        if routable.route.as_bytes() != decoded_route.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: routable.route.to_string(),
                decoded: String::from_utf8_lossy(&decoded_route).to_string(),
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
    /// use wayfind::{Constraint, Path, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    ///
    /// let path = Path::new("/hello").unwrap();
    /// let search = router.search(&path).unwrap();
    /// ```
    pub fn search<'p>(
        &'r self,
        path: &'p Path<'_>,
    ) -> Result<Option<Match<'r, 'p, T>>, SearchError> {
        let mut parameters = smallvec![];
        let Some((data, _)) =
            self.root
                .search(path.as_bytes(), &mut parameters, &self.constraints)?
        else {
            return Ok(None);
        };

        let (route, expanded, data) = match data {
            Data::Inline { route, value, .. } => (route, None, value),
            Data::Shared {
                route,
                expanded,
                value,
                ..
            } => (route, Some(expanded.as_ref()), value.as_ref()),
        };

        Ok(Some(Match {
            route,
            expanded,
            data,
            parameters,
        }))
    }
}

impl<'r, T> Default for Router<'r, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, T> Display for Router<'r, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.root)
    }
}
