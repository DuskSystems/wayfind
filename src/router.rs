use crate::{
    node::{Node, NodeData, NodeKind},
    segment::Segments,
};

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
            },
        }
    }

    pub fn insert(&mut self, path: &'a str, value: T) {
        self.root
            .insert(Segments::new(path.as_bytes()), NodeData { path, value });
    }
}

impl<'a, T> Default for Router<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
