use crate::{
    id::RouteId,
    storage::{Storage, StorageKind},
    Request, Route,
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
use errors::{constraint::PathConstraintError, PathDeleteError, PathInsertError, PathSearchError};
use hashbrown::HashMap;
use node::{state::RootState, Children, Data, Node};
use parser::{Parser, Part};
use smallvec::{smallvec, SmallVec};

pub mod constraints;
pub mod errors;
pub mod node;
pub mod parser;

pub use constraints::PathConstraint;

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct PathMatch<'r, 'p, T> {
    /// The matching route.
    pub route: &'r str,

    /// The expanded route, if applicable.
    pub expanded: Option<&'r str>,

    /// A reference to the matching route data.
    pub data: &'r T,

    /// Key-value pairs of parameters, extracted from the route.
    pub parameters: PathParameters<'r, 'p>,
}

/// All the parameter pairs of a given match.
///
/// The key of the parameter is tied to the lifetime of the router, since it is a ref to the prefix of a given node.
/// Meanwhile, the value is extracted from the path.
pub type PathParameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

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
pub struct PathRouter<'r, T> {
    /// The root node of the tree.
    root: Node<'r, T, RootState>,

    /// A map of constraint names to [`StoredConstraint`].
    constraints: HashMap<&'r str, StoredConstraint>,
}

impl<'r, T> PathRouter<'r, T> {
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
    /// use wayfind::{PathConstraint, Router};
    ///
    /// struct HelloConstraint;
    /// impl PathConstraint for HelloConstraint {
    ///     const NAME: &'static str = "hello";
    ///
    ///     fn check(segment: &str) -> bool {
    ///         segment == "hello"
    ///     }
    /// }
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.path.constraint::<HelloConstraint>().unwrap();
    /// ```
    pub fn constraint<C: PathConstraint>(&mut self) -> Result<(), PathConstraintError> {
        if let Some(existing) = self.constraints.get(C::NAME) {
            return Err(PathConstraintError::DuplicateName {
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
    /// use wayfind::{PathConstraint, Router, RouteBuilder};
    ///
    /// let mut router: Router<usize> = Router::new();
    ///
    /// let route = RouteBuilder::new()
    ///     .route("/hello/{world}")
    ///     .build()
    ///     .unwrap();
    /// router.insert(&route, 1).unwrap();
    /// ```
    pub(crate) fn insert(
        &mut self,
        route: &Route<'r>,
        value: Option<T>,
    ) -> Result<(), PathInsertError> {
        let mut parsed = Parser::new(route.route.as_bytes())?;
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
                        return Err(PathInsertError::UnknownConstraint {
                            constraint: name.to_string(),
                        });
                    }
                }
            }
        }

        let storage = match route.storage {
            StorageKind::Inline => Storage::Inline(value.unwrap()),
            StorageKind::Router(id) => Storage::Router(id),
        };

        if parsed.routes.len() > 1 {
            let mut errors = vec![];
            let storage = Arc::new(storage);

            for mut parsed_route in parsed.routes {
                let expanded = Arc::from(String::from_utf8_lossy(&parsed_route.raw));

                if let Err(err) = self.root.insert(
                    &mut parsed_route,
                    Data::Shared {
                        route: route.route,
                        expanded,
                        storage: Arc::clone(&storage),
                    },
                ) {
                    errors.push(err);
                }
            }

            if !errors.is_empty() {
                drop(self.delete(route));
                errors.dedup();

                if errors.len() == 1 {
                    let error = errors.remove(0);
                    return Err(error);
                }

                return Err(PathInsertError::Multiple(errors));
            }
        } else if let Some(parsed_route) = parsed.routes.first_mut() {
            self.root.insert(
                parsed_route,
                Data::Inline {
                    route: route.route,
                    storage,
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
    /// use wayfind::{PathConstraint, Router, RouteBuilder};
    ///
    /// let mut router: Router<usize> = Router::new();
    ///
    /// let route = RouteBuilder::new()
    ///     .route("/hello")
    ///     .build()
    ///     .unwrap();
    ///
    /// router.insert(&route, 1).unwrap();
    /// router.delete(&route).unwrap();
    /// ```
    pub(crate) fn delete(&mut self, route: &Route<'r>) -> Result<Data<'r, T>, PathDeleteError> {
        let mut parsed = Parser::new(route.route.as_bytes())?;

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

                return Err(PathDeleteError::Multiple(errors));
            }

            data.unwrap()
        } else {
            let route = parsed.routes.first_mut().unwrap();
            self.root.delete(route, false)?
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
    /// use wayfind::{Router, RouteBuilder, RequestBuilder};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// let route = RouteBuilder::new()
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
    pub(crate) fn search<'p>(
        &'r self,
        request: &'p Request<'p>,
        map: &'r HashMap<RouteId, T>,
    ) -> Result<Option<PathMatch<'r, 'p, T>>, PathSearchError> {
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
                let Some(data) = map.get(id) else {
                    return Ok(None);
                };

                data
            }
        };

        Ok(Some(PathMatch {
            route,
            expanded,
            data,
            parameters,
        }))
    }
}

impl<'r, T> Default for PathRouter<'r, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, T> Display for PathRouter<'r, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.root)
    }
}