use crate::{
    matches::Match,
    node::{Node, NodeData, NodeKind},
    segment::Segments,
};
use smallvec::smallvec;

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

                quick_dynamic: false,
            },
        }
    }

    pub fn insert(&mut self, path: &'a str, value: T) {
        self.root
            .insert(Segments::new(path.as_bytes()), NodeData { path, value });
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
