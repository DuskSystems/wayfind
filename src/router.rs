use crate::{
    errors::{delete::DeleteError, insert::InsertError},
    matches::Match,
    node::{Node, NodeConstraint, NodeData, NodeKind},
    parts::Parts,
    route::Route,
};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Router<T> {
    root: Node<T>,
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
                regex_children: vec![],
                dynamic_children: vec![],
                wildcard_children: vec![],
                end_wildcard: None,

                quick_regex: false,
                quick_dynamic: false,
            },
        }
    }

    pub fn insert(&mut self, path: &str, value: T) -> Result<(), InsertError> {
        let mut route = Route::new(path, vec![])?;
        let path = Arc::from(route.path);

        self.root
            .insert(&mut route, NodeData { path, value })
    }

    pub fn insert_with_constraints(
        &mut self,
        path: &str,
        value: T,
        constraints: Vec<(&str, NodeConstraint)>,
    ) -> Result<(), InsertError> {
        let mut route = Route::new(path, constraints)?;
        let path = Arc::from(route.path);

        self.root
            .insert(&mut route, NodeData { path, value })
    }

    pub fn delete(&mut self, path: &str) -> Result<(), DeleteError> {
        let mut parts = Parts::new(path.as_bytes())?;
        self.root.delete(&mut parts)
    }

    #[must_use]
    pub fn matches<'a>(&'a self, path: &'a str) -> Option<Match<'a, T>> {
        let mut parameters = vec![];
        let data = self
            .root
            .matches(path.as_bytes(), &mut parameters)?;

        Some(Match { data, parameters })
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
