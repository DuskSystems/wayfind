use super::PathData;
use state::{DynamicState, EndWildcardState, State, StaticState, WildcardState};
use std::ops::{Index, IndexMut};

pub mod delete;
pub mod display;
pub mod insert;
pub mod optimize;
pub mod search;
pub mod state;

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<'r, S: State> {
    /// The type of Node, and associated structure data.
    pub state: S,

    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<PathData<'r>>,

    pub static_children: Children<'r, StaticState>,
    pub dynamic_children: Children<'r, DynamicState>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Children<'r, WildcardState>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Children<'r, EndWildcardState>,

    /// Higher values indicate more specific matches.
    pub priority: usize,
    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}

/// A list of node children.
/// Maintains whether it is sorted automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Children<'r, S: State> {
    nodes: Vec<Node<'r, S>>,
    sorted: bool,
}

impl<'r, S: State> Children<'r, S> {
    const fn new(nodes: Vec<Node<'r, S>>) -> Self {
        Self {
            nodes,
            sorted: false,
        }
    }

    fn push(&mut self, node: Node<'r, S>) {
        self.nodes.push(node);
        self.sorted = false;
    }

    fn remove(&mut self, index: usize) -> Node<'r, S> {
        self.nodes.remove(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<'r, S>>
    where
        F: Fn(&Node<'r, S>) -> bool,
    {
        self.nodes.iter_mut().find(|node| predicate(node))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Node<'r, S>> {
        self.nodes.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<'r, S>> {
        self.nodes.iter_mut()
    }
}

impl<S: State + Ord> Children<'_, S> {
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

impl<S: State> Default for Children<'_, S> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            sorted: false,
        }
    }
}

impl<'r, S: State> Index<usize> for Children<'r, S> {
    type Output = Node<'r, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<S: State> IndexMut<usize> for Children<'_, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
