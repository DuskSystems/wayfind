use smallvec::smallvec;

use crate::{
    errors::{delete::DeleteError, insert::InsertError},
    matches::Match,
    node::{Node, NodeConstraint, NodeData, NodeKind},
    route::Route,
};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Router<'a, T> {
    root: Node<'a, T>,
}

impl<'a, T> Router<'a, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node {
                kind: NodeKind::Root,
                prefix: b"",
                data: None,
                constraint: None,
                static_children: vec![],
                dynamic_children: vec![],
                wildcard_children: vec![],
                end_wildcard: None,
                quick_dynamic: false,
            },
        }
    }

    pub fn insert(&mut self, path: &'a str, value: T) -> Result<(), InsertError> {
        let mut route = Route::new(path, vec![])?;
        let path = Arc::from(route.path);

        self.root
            .insert(&mut route, NodeData { path, value })
    }

    pub fn insert_with_constraints(
        &mut self,
        path: &'a str,
        value: T,
        constraints: Vec<(&'a str, impl Into<NodeConstraint>)>,
    ) -> Result<(), InsertError> {
        let constraints = constraints
            .into_iter()
            .map(|(name, constraint)| (name, constraint.into()))
            .collect();

        let mut route = Route::new(path, constraints)?;
        let path = Arc::from(route.path);

        self.root
            .insert(&mut route, NodeData { path, value })
    }

    pub fn delete(&mut self, path: &str) -> Result<(), DeleteError> {
        let mut route = Route::new(path, vec![])?;
        self.root.delete(&mut route)
    }

    pub fn delete_with_constraints(
        &mut self,
        path: &str,
        constraints: Vec<(&str, impl Into<NodeConstraint>)>,
    ) -> Result<(), DeleteError> {
        let constraints = constraints
            .into_iter()
            .map(|(name, constraint)| (name, constraint.into()))
            .collect();

        let mut route = Route::new(path, constraints)?;
        self.root.delete(&mut route)
    }

    #[must_use]
    pub fn matches(&'a self, path: &'a str) -> Option<Match<'a, T>> {
        let mut parameters = smallvec![];
        let data = self
            .root
            .matches(path.as_bytes(), &mut parameters)?;

        Some(Match { data, parameters })
    }
}

impl<'a, T> Default for Router<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Display> Display for Router<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
