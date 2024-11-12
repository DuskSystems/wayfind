use crate::state::{DynamicState, EndWildcardState, State, StaticState, WildcardState};
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
pub struct Node<'r, T, S: State> {
    /// The type of Node, and associated structure data.
    pub state: S,

    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<Data<'r, T>>,

    pub static_children: Children<'r, T, StaticState>,
    pub dynamic_children: Children<'r, T, DynamicState>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Children<'r, T, WildcardState>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Children<'r, T, EndWildcardState>,

    /// Higher values indicate more specific matches.
    pub priority: usize,
    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}

/// Holds data associated with a given node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data<'r, T> {
    /// Data is stored inline.
    Inline {
        /// The original route.
        route: &'r str,

        /// The associated data.
        value: T,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The original route.
        route: &'r str,

        /// The expanded route.
        expanded: Arc<str>,

        /// The associated data, shared.
        value: Arc<T>,
    },
}

/// A list of node children.
/// Maintains whether it is sorted automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Children<'r, T, S: State> {
    nodes: Vec<Node<'r, T, S>>,
    sorted: bool,
}

impl<'r, T, S: State> Children<'r, T, S> {
    fn push(&mut self, node: Node<'r, T, S>) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn remove(&mut self, index: usize) -> Node<'r, T, S> {
        self.nodes.remove(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<'r, T, S>>
    where
        F: Fn(&Node<'r, T, S>) -> bool,
    {
        self.nodes.iter_mut().find(|node| predicate(node))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Node<'r, T, S>> {
        self.nodes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<'r, T, S>> {
        self.nodes.iter_mut()
    }
}

impl<'r, T, S: State + Ord> Children<'r, T, S> {
    fn sort(&mut self) {
        if self.sorted {
            return;
        }

        self.nodes.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.state.cmp(&b.state))
        });

        self.sorted = true;
    }
}

impl<'r, T, S: State> Default for Children<'r, T, S> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            sorted: false,
        }
    }
}

impl<'r, T, S: State> From<Vec<Node<'r, T, S>>> for Children<'r, T, S> {
    fn from(value: Vec<Node<'r, T, S>>) -> Self {
        Self {
            nodes: value,
            sorted: false,
        }
    }
}

impl<'r, T, S: State> Index<usize> for Children<'r, T, S> {
    type Output = Node<'r, T, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'r, T, S: State> IndexMut<usize> for Children<'r, T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
