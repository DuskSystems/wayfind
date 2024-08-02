use crate::{
    matches::Match,
    node::{Node, NodeData, NodeKind},
    parts::Parts,
};
use smallvec::smallvec;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
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

                static_children: vec![],
                dynamic_children: vec![],
                end_wildcard: None,

                quick_dynamic: false,
            },
        }
    }

    pub fn insert(&mut self, path: &'a str, value: T) {
        self.root.insert(
            Parts::new(path.as_bytes()),
            NodeData {
                path: path.into(),
                value,
            },
        );
    }

    #[must_use]
    pub fn matches(&'a self, path: &'a str) -> Option<Match<T>> {
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
