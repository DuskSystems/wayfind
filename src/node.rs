use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    sync::Arc,
};

pub mod delete;
pub mod display;
pub mod insert;
pub mod optimize;
pub mod search;

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub kind: Kind,

    /// The prefix may either be the static bytes of a path, or the name of a variable.
    pub prefix: Vec<u8>,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<Data<T>>,
    /// An optional check to run, to restrict routing to this node.
    pub constraint: Option<Vec<u8>>,

    pub static_children: Children<T>,
    pub dynamic_children: Children<T>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Children<T>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Children<T>,

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
pub enum Data<T> {
    /// Data is stored inline.
    Inline {
        /// The original route.
        route: Arc<str>,

        /// The associated data.
        value: T,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The original route.
        route: Arc<str>,

        /// The expanded route.
        expanded: Arc<str>,

        /// The associated data, shared.
        value: Arc<T>,
    },
}

/// A list of node children.
/// Maintains whether it is sorted automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Children<T> {
    nodes: Vec<Node<T>>,
    sorted: bool,
}

impl<T> Children<T> {
    fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn remove(&mut self, index: usize) -> Node<T> {
        self.nodes.remove(index)
    }

    fn sort(&mut self) {
        if self.sorted {
            return;
        }

        self.nodes.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.prefix.cmp(&b.prefix))
                .then_with(|| a.constraint.cmp(&b.constraint))
        });

        self.sorted = true;
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<T>>
    where
        F: Fn(&Node<T>) -> bool,
    {
        self.nodes.iter_mut().find(|node| predicate(node))
    }

    fn iter(&self) -> impl Iterator<Item = &Node<T>> {
        self.nodes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<T>> {
        self.nodes.iter_mut()
    }
}

impl<T> Default for Children<T> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            sorted: true,
        }
    }
}

impl<T> From<Vec<Node<T>>> for Children<T> {
    fn from(value: Vec<Node<T>>) -> Self {
        Self {
            nodes: value,
            sorted: false,
        }
    }
}

impl<T> Index<usize> for Children<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T> IndexMut<usize> for Children<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
