use std::{fmt::Debug, sync::Arc};

pub mod delete;
pub mod display;
pub mod insert;
pub mod search;

/// A node in the router tree structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// The root node of the tree.
    /// Must only be used in the router new method.
    Root,

    /// A static node with a fixed path segment.
    Static,

    /// A dynamic node that can match any bytes, excluding b'/'.
    Dynamic,

    /// A wildcard node that can match any bytes, including b'/'.
    Wildcard,

    /// An end wildcard node that matches the whole remaining path.
    /// Must only exist at the end of a tree.
    EndWildcard,
}

/// Holds data associated with a given node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData<T> {
    /// The full path from the root to this node.
    pub path: Arc<str>,
    /// The value associated with this node.
    pub value: T,
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
