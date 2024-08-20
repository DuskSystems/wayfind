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

#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

#[derive(Clone)]
pub struct Router<T> {
    root: Node<T>,
    constraints: HashMap<Vec<u8>, StoredConstraint>,
}

impl<T> Router<T> {
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

    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
        if route.as_bytes() != Path::new(route)?.decoded_bytes() {
            return Err(InsertError::EncodedPath);
        }

        let path = Arc::from(route);
        let mut parts = Parts::new(route.as_bytes())?;

        for part in &parts.0 {
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
                    return Err(InsertError::UnknownConstraint);
                }
            }
        }

        self.root.insert(&mut parts, NodeData { path, value })
    }

    pub fn delete(&mut self, route: &str) -> Result<(), DeleteError> {
        let mut parts = Parts::new(route.as_bytes())?;
        self.root.delete(&mut parts)
    }

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
