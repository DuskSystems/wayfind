use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    sync::Arc,
};

use crate::id::RoutableId;

pub mod delete;
pub mod display;
pub mod find;
pub mod insert;
pub mod optimize;
pub mod search;

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub kind: Kind,

    /// The prefix may either be the static bytes of a path, or the name of a variable.
    pub prefix: Vec<u8>,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<Data>,
    /// An optional check to run, to restrict routing to this node.
    pub constraint: Option<Vec<u8>>,

    pub static_children: Children,
    pub dynamic_children: Children,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Children,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Children,

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
pub enum Data {
    /// Data is stored inline.
    Inline {
        /// The data lookup key.
        id: RoutableId,

        /// The original route.
        route: Arc<str>,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The data lookup key.
        id: RoutableId,

        /// The original route.
        route: Arc<str>,

        /// The expanded route.
        expanded: Arc<str>,
    },
}

impl Data {
    pub const fn id(&self) -> RoutableId {
        match self {
            Self::Inline { id, .. } | Self::Shared { id, .. } => *id,
        }
    }

    pub fn route(&self) -> Arc<str> {
        match self {
            Self::Inline { route, .. } | Self::Shared { route, .. } => Arc::clone(route),
        }
    }

    pub fn expanded(&self) -> Option<Arc<str>> {
        match self {
            Self::Inline { .. } => None,
            Self::Shared { expanded, .. } => Some(Arc::clone(expanded)),
        }
    }
}

/// A list of node children.
/// Maintains whether it is sorted automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Children {
    nodes: Vec<Node>,
    sorted: bool,
}

impl Children {
    fn push(&mut self, node: Node) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn remove(&mut self, index: usize) -> Node {
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

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node>
    where
        F: Fn(&Node) -> bool,
    {
        self.nodes.iter_mut().find(|node| predicate(node))
    }

    fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.iter_mut()
    }
}

impl Default for Children {
    fn default() -> Self {
        Self {
            nodes: vec![],
            sorted: true,
        }
    }
}

impl From<Vec<Node>> for Children {
    fn from(value: Vec<Node>) -> Self {
        Self {
            nodes: value,
            sorted: false,
        }
    }
}

impl Index<usize> for Children {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Children {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
