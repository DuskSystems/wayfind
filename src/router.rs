use crate::{
    errors::{delete::DeleteError, insert::InsertError},
    matches::Match,
    node::{Node, NodeData, NodeKind},
    route::IntoRoute,
};
use smallvec::smallvec;
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
                constraints: smallvec![],

                static_children: vec![],
                dynamic_children: vec![],
                wildcard_children: vec![],
                end_wildcard_children: vec![],

                quick_dynamic: false,
            },
        }
    }

    pub fn insert<'a, R>(&mut self, route: R, value: T) -> Result<(), InsertError>
    where
        R: IntoRoute<'a>,
    {
        let mut route = route.into_route()?;
        let path = Arc::from(route.path);

        self.root
            .insert(&mut route, NodeData { path, value })
    }

    pub fn delete<'a, R>(&mut self, route: R) -> Result<(), DeleteError>
    where
        R: IntoRoute<'a>,
    {
        let mut route = route.into_route()?;
        self.root.delete(&mut route)
    }

    #[must_use]
    pub fn matches<'a>(&'a self, path: &'a str) -> Option<Match<'a, T>> {
        let mut parameters = smallvec![];
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
