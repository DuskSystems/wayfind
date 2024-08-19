use crate::{
    constraints::Constraint,
    errors::{
        constraint::ConstraintError, delete::DeleteError, insert::InsertError, search::SearchError,
    },
    node::{search::Match, Node, NodeData, NodeKind},
    parts::{Part, Parts},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

#[derive(Clone)]
pub struct Router<T> {
    root: Node<T>,
    constraints: HashMap<Vec<u8>, fn(&str) -> bool>,
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
                entry.insert(C::check);
                Ok(())
            }
            Entry::Occupied(_) => Err(ConstraintError::DuplicateName),
        }
    }

    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
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

    pub fn search<'k, 'v>(
        &'k self,
        path: &'v str,
    ) -> Result<Option<Match<'k, 'v, T>>, SearchError> {
        let mut parameters = vec![];
        let Some(node) = self
            .root
            .search(path.as_bytes(), &mut parameters, &self.constraints)?
        else {
            return Ok(None);
        };

        let Some(data) = node.data.as_ref() else {
            return Ok(None);
        };

        Ok(Some(Match { data, parameters }))
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
