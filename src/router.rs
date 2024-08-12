use crate::{
    errors::{delete::DeleteError, insert::InsertError},
    matches::Match,
    node::{Constraint, Node, NodeConstraint, NodeData, NodeKind},
    parts::Parts,
};
use smallvec::smallvec;
use std::{collections::HashMap, error::Error, fmt::Display, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Router<T> {
    root: Node<T>,
    constraints: HashMap<Arc<str>, NodeConstraint>,
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

    // FIXME: Error for duplicates?
    pub fn constraint<C: Constraint>(&mut self) -> Result<(), Box<dyn Error>> {
        self.constraints.insert(
            Arc::from(C::name()),
            NodeConstraint {
                name: C::name(),
                check: C::check,
            },
        );

        Ok(())
    }

    pub fn insert(&mut self, route: &str, value: T) -> Result<(), InsertError> {
        let path = Arc::from(route);
        let mut parts = Parts::new(route.as_bytes())?;

        // TODO: Check all constraints are valid up-front?

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
