use crate::{
    constraint::{Constraint, NodeConstraint},
    errors::{constraint::ConstraintError, delete::DeleteError, insert::InsertError},
    matches::Match,
    node::{Node, NodeData, NodeKind},
    parts::{Part, Parts},
};
use smallvec::smallvec;
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    sync::Arc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Router<T> {
    root: Node<T>,
    constraints: HashMap<Vec<u8>, NodeConstraint>,
}

impl<T> Router<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
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
        }
    }

    pub fn constraint<C: Constraint>(&mut self) -> Result<(), ConstraintError> {
        match self
            .constraints
            .entry(C::NAME.as_bytes().to_vec())
        {
            Entry::Vacant(entry) => {
                entry.insert(NodeConstraint {
                    name: C::NAME,
                    check: C::check,
                });

                Ok(())
            }
            Entry::Occupied(_) => Err(ConstraintError::DuplicateName),
        }
    }

    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
        let path = Arc::from(route);
        let mut parts = Parts::new(route.as_bytes())?;

        for part in &parts.0 {
            match part {
                Part::Dynamic {
                    constraint: Some(name), ..
                }
                | Part::Wildcard {
                    constraint: Some(name), ..
                } => {
                    if !self.constraints.contains_key(name) {
                        return Err(InsertError::UnknownConstraint);
                    }
                }
                _ => (),
            }
        }

        self.root
            .insert(&mut parts, NodeData { path, value })
    }

    pub fn delete(&mut self, route: &str) -> Result<(), DeleteError> {
        let mut parts = Parts::new(route.as_bytes())?;
        self.root.delete(&mut parts)
    }

    #[must_use]
    pub fn matches<'k, 'v>(&'k self, path: &'v str) -> Option<Match<'k, 'v, T>> {
        let mut parameters = smallvec![];
        let node = self
            .root
            .matches(path.as_bytes(), &mut parameters, &self.constraints)?;

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
