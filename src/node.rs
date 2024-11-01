use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

pub mod delete;
pub mod display;
pub mod insert;
pub mod optimize;
pub mod search;

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<'router, T> {
    pub kind: Kind,

    /// The prefix may either be the static bytes of a path, or the name of a variable.
    pub prefix: &'router [u8],
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<Data<'router, T>>,

    pub static_children: Children<'router, T>,
    pub dynamic_children: Children<'router, T>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Children<'router, T>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Children<'router, T>,

    /// Higher values indicate more specific matches.
    pub priority: usize,
    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}

/// A node in the router tree structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The root node of the tree.
    /// Must only be used in the [`Router::new`](crate::Router::new) method.
    Root,

    /// A node with a fixed path segment.
    Static,

    /// A node that can match any bytes, excluding b'/'.
    Dynamic,

    /// A node that can match any bytes, including b'/'.
    Wildcard,

    /// A node that matches the whole remaining path.
    EndWildcard,
}

/// Holds data associated with a given node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data<'router, T> {
    /// The original route.
    pub(crate) route: &'router str,

    /// The associated data.
    pub(crate) value: T,
}

/// A list of node children.
/// Maintains whether it is sorted automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Children<'router, T> {
    nodes: Vec<Node<'router, T>>,
    sorted: bool,
}

impl<'router, T> Children<'router, T> {
    fn push(&mut self, node: Node<'router, T>) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn remove(&mut self, index: usize) -> Node<'router, T> {
        self.nodes.remove(index)
    }

    fn sort(&mut self) {
        if self.sorted {
            return;
        }

        self.nodes.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.prefix.cmp(b.prefix))
        });

        self.sorted = true;
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<'router, T>>
    where
        F: Fn(&Node<'router, T>) -> bool,
    {
        self.nodes.iter_mut().find(|node| predicate(node))
    }

    fn iter(&self) -> impl Iterator<Item = &Node<'router, T>> {
        self.nodes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<'router, T>> {
        self.nodes.iter_mut()
    }
}

impl<'router, T> Default for Children<'router, T> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            sorted: true,
        }
    }
}

impl<'router, T> From<Vec<Node<'router, T>>> for Children<'router, T> {
    fn from(value: Vec<Node<'router, T>>) -> Self {
        Self {
            nodes: value,
            sorted: false,
        }
    }
}

impl<'router, T> Index<usize> for Children<'router, T> {
    type Output = Node<'router, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'router, T> IndexMut<usize> for Children<'router, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
