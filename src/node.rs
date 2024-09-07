use std::{fmt::Debug, sync::Arc};

pub mod delete;
pub mod display;
pub mod insert;
pub mod search;

/// A node in the router tree structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
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
pub enum NodeData<T> {
    /// Data is stored inline.
    Inline {
        /// The original route path.
        route: Arc<str>,

        /// The associated data.
        value: T,
    },

    /// Data is stored at the router level, as it's shared between 2 or more nodes.
    #[allow(dead_code)]
    Reference(Arc<str>),
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub kind: NodeKind,

    /// The prefix may either be the static bytes of a path, or the name of a variable.
    pub prefix: Vec<u8>,
    /// Optional data associated with this node.
    /// The presense of this data is needed to successfully match a route.
    pub data: Option<NodeData<T>>,
    /// An optional check to run, to restrict routing to this node.
    pub constraint: Option<Vec<u8>>,

    pub static_children: Vec<Node<T>>,
    pub dynamic_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
    pub end_wildcard_children: Vec<Node<T>>,

    /// A flag indicating whether this node's dynamic children can be matched quickly.
    /// This allows us to traverse the next section of the path by segment, rather than byte-by-byte, when matching.
    pub quick_dynamic: bool,
}
