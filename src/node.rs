use crate::id::RoutableId;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    sync::Arc,
};

pub mod delete;
pub mod display;
pub mod find;
pub mod insert;
pub mod optimize;
pub mod search;

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<'router> {
    pub kind: Kind,

    /// The prefix may either be the static bytes of a path, or the name of a variable.
    pub prefix: Vec<u8>,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<Data<'router>>,
    /// An optional check to run, to restrict routing to this node.
    pub constraint: Option<Vec<u8>>,

    pub static_children: Children<'router>,
    pub dynamic_children: Children<'router>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Children<'router>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Children<'router>,

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
pub enum Data<'a> {
    /// Data is stored inline.
    Inline {
        /// The data lookup key.
        id: RoutableId,

        /// The original route.
        route: &'a str,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The data lookup key.
        id: RoutableId,

        /// The original route.
        route: &'a str,

        /// The expanded route.
        expanded: Arc<str>,
    },
}

impl<'a> Data<'a> {
    pub const fn id(&self) -> RoutableId {
        match self {
            Self::Inline { id, .. } | Self::Shared { id, .. } => *id,
        }
    }

    pub const fn route(&self) -> &'a str {
        match self {
            Self::Inline { route, .. } | Self::Shared { route, .. } => route,
        }
    }

    pub fn expanded(&'a self) -> Option<&'a str> {
        match self {
            Self::Inline { .. } => None,
            Self::Shared { expanded, .. } => Some(expanded),
        }
    }
}

/// A list of node children.
/// Maintains whether it is sorted automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Children<'a> {
    nodes: Vec<Node<'a>>,
    sorted: bool,
}

impl<'a> Children<'a> {
    fn push(&mut self, node: Node<'a>) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn remove(&mut self, index: usize) -> Node<'a> {
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

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<'a>>
    where
        F: Fn(&Node<'a>) -> bool,
    {
        self.nodes.iter_mut().find(|node| predicate(node))
    }

    fn iter(&self) -> impl Iterator<Item = &Node<'a>> {
        self.nodes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<'a>> {
        self.nodes.iter_mut()
    }
}

impl<'a> Default for Children<'a> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            sorted: true,
        }
    }
}

impl<'a> From<Vec<Node<'a>>> for Children<'a> {
    fn from(value: Vec<Node<'a>>) -> Self {
        Self {
            nodes: value,
            sorted: false,
        }
    }
}

impl<'a> Index<usize> for Children<'a> {
    type Output = Node<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'a> IndexMut<usize> for Children<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
