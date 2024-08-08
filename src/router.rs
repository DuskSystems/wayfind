use crate::{
    errors::{delete::DeleteError, insert::InsertError},
    matches::Match,
    node::{Node, NodeData, NodeKind},
    route::IntoRoute,
};
use smallvec::smallvec;
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Router<T, R> {
    root: Node<T, R>,
}

impl<T, R> Router<T, R> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node {
                kind: NodeKind::Root,

                prefix: vec![],
                data: None,

                parameter_constraints: vec![],
                request_constraints: vec![],

                static_children: vec![],
                dynamic_children: vec![],
                wildcard_children: vec![],
                end_wildcard_children: vec![],

                quick_dynamic: false,
            },
        }
    }

    pub fn insert<'a, RR>(&mut self, route: RR, value: T) -> Result<(), InsertError>
    where
        RR: IntoRoute<'a, R>,
    {
        let mut route = route.into_route()?;
        let path = Arc::from(route.path);

        self.root
            .insert(&mut route, NodeData { path, value })
    }

    pub fn delete<'a, RR>(&mut self, route: RR) -> Result<(), DeleteError>
    where
        RR: IntoRoute<'a, R>,
    {
        let mut route = route.into_route()?;
        self.root.delete(&mut route)
    }

    #[must_use]
    pub fn matches<'k, 'v>(&'k self, path: &'v str) -> Option<Match<'k, 'v, T>> {
        let mut parameters = smallvec![];

        let node = self
            .root
            .path_matches(path.as_bytes(), &mut parameters)?;

        Some(Match {
            data: node.data.as_ref()?,
            parameters,
        })
    }
}

impl<T, R> Default for Router<T, R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display, R> Display for Router<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
