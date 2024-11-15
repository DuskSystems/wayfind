use crate::{
    constraints::Constraint,
    errors::{ConstraintError, DeleteError, InsertError, SearchError},
    id::RouteId,
    node::{Children, Data, Node},
    parser::{Parser, Part},
    state::RootState,
    storage::{Storage, StorageKind},
    Request, Routable,
};
use alloc::{
    fmt::Display,
    string::{String, ToString},
    sync::Arc,
    vec,
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

    /// Stored data.
    data: HashMap<RouteId, T>,
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
            data: HashMap::default(),
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
        if let Some(existing) = self.constraints.get(C::NAME) {
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
    /// use wayfind::{Constraint, Router, RoutableBuilder};
    ///
    /// let mut router: Router<usize> = Router::new();
    ///
    /// let route = RoutableBuilder::new()
    ///     .route("/hello/{world}")
    ///     .build()
    ///     .unwrap();
    /// router.insert(&route, 1).unwrap();
    /// ```
    pub fn insert(&mut self, routable: &Routable<'r>, value: T) -> Result<(), InsertError> {
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

        let storage = match routable.storage {
            StorageKind::Inline => Storage::Inline(value),

            // FIXME: This should be cleaned up on failure. Don't worry about that for now.
            StorageKind::Router => {
                let id = RouteId::new();
                self.data.insert(id, value);
                Storage::Router(id)
            }
        };

        if parsed.routes.len() > 1 {
            let mut errors = vec![];
            let storage = Arc::new(storage);

            for mut route in parsed.routes {
                let expanded = Arc::from(String::from_utf8_lossy(&route.raw));

                if let Err(err) = self.root.insert(
                    &mut route,
                    Data::Shared {
                        route: routable.route,
                        expanded,
                        storage: Arc::clone(&storage),
                    },
                ) {
                    errors.push(err);
                }
            }

            if !errors.is_empty() {
                drop(self.delete(routable));
                errors.dedup();

                if errors.len() == 1 {
                    let error = errors.remove(0);
                    return Err(error);
                }

                return Err(InsertError::Multiple(errors));
            }
        } else if let Some(route) = parsed.routes.first_mut() {
            self.root.insert(
                route,
                Data::Inline {
                    route: routable.route,
                    storage,
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
    /// use wayfind::{Constraint, Router, RoutableBuilder};
    ///
    /// let mut router: Router<usize> = Router::new();
    ///
    /// let route = RoutableBuilder::new()
    ///     .route("/hello")
    ///     .build()
    ///     .unwrap();
    ///
    /// router.insert(&route, 1).unwrap();
    /// router.delete(&route).unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// If the parser returns zero routes, which should never happen.
    pub fn delete(&mut self, routable: &Routable<'r>) -> Result<Data<'r, T>, DeleteError> {
        let mut parsed = Parser::new(routable.route.as_bytes())?;

        let data = if parsed.routes.len() > 1 {
            let mut data: Option<Data<'r, T>> = None;
            let mut errors = vec![];

            for mut expanded_route in parsed.routes {
                match self.root.delete(&mut expanded_route, true) {
                    Ok(result) => data = Some(result),
                    Err(err) => errors.push(err),
                }
            }

            if !errors.is_empty() {
                errors.dedup();

                if errors.len() == 1 {
                    let error = errors.remove(0);
                    return Err(error);
                }

                return Err(DeleteError::Multiple(errors));
            }

            data.unwrap()
        } else {
            let route = parsed.routes.first_mut().unwrap();
            self.root.delete(route, false)?
        };

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

        self.root.optimize();
        Ok(data)
    }

    /// Searches for a matching [`Request`] in the [`Router`].
    ///
    /// # Errors
    ///
    /// Returns a [`SearchError`] if the search resulted in invalid parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Router, RoutableBuilder, RequestBuilder};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// let route = RoutableBuilder::new()
    ///     .route("/hello")
    ///     .build()
    ///     .unwrap();
    /// router.insert(&route, 1).unwrap();
    ///
    /// let request = RequestBuilder::new()
    ///     .path("/hello")
    ///     .build()
    ///     .unwrap();
    /// let search = router.search(&request).unwrap();
    /// ```
    pub fn search<'p>(
        &'r self,
        request: &'p Request<'p>,
    ) -> Result<Option<Match<'r, 'p, T>>, SearchError> {
        let mut parameters = smallvec![];
        let Some((data, _)) =
            self.root
                .search(request.path.as_ref(), &mut parameters, &self.constraints)?
        else {
            return Ok(None);
        };

        let (storage, route, expanded) = match data {
            Data::Inline { storage, route, .. } => (storage, route, None),
            Data::Shared {
                storage,
                route,
                expanded,
                ..
            } => (storage.as_ref(), route, Some(expanded.as_ref())),
        };

        let data = match storage {
            Storage::Inline(data) => data,
            Storage::Router(id) => {
                let Some(data) = self.data.get(id) else {
                    return Ok(None);
                };

                data
            }
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
