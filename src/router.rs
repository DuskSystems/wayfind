use crate::{
    constraints::Constraint,
    errors::{constraint::ConstraintError, delete::DeleteError, insert::InsertError},
    node::{search::Match, Node, NodeData, NodeKind},
    parts::{Part, Parts},
    path::Path,
};
use smallvec::smallvec;
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
    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
        let path = Path::new(route)?;
        if route.as_bytes() != path.decoded_bytes() {
            return Err(InsertError::EncodedPath {
                input: route.to_string(),
                decoded: String::from_utf8_lossy(path.decoded_bytes()).to_string(),
            });
        }

        let path = Arc::from(route);
        let mut parts = Parts::new(route.as_bytes())?;

        for part in &parts {
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

        self.root.insert(&mut parts, NodeData { path, value })
    }

    /// Deletes a route from the router.
    ///
    /// The route provided must exactly match the route inserted.
    ///
    /// # Errors
    ///
    /// Returns a [`DeleteError`] if the route cannot be deleted, or cannot be found.
    pub fn delete(&mut self, route: &str) -> Result<(), DeleteError> {
        let mut parts = Parts::new(route.as_bytes())?;
        self.root.delete(&mut parts)
    }

    /// Searches for a matching route in the router.
    ///
    /// Returns a [`Match`] if a matching route is found, or [`None`] otherwise.
    pub fn search<'router, 'path>(
        &'router self,
        path: &'path Path,
    ) -> Option<Match<'router, 'path, T>> {
        let mut parameters = smallvec![];
        let node = self
            .root
            .search(path.decoded_bytes(), &mut parameters, &self.constraints)?;

        Some(Match {
            data: node.data.as_ref()?,
            parameters,
        })
    }
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display> Display for Router<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
