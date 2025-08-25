use alloc::{boxed::Box, string::String, vec::Vec};
use core::cmp::Ordering;

use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod conflict;
mod delete;
mod display;
mod find;
mod insert;
mod optimize;
mod search;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData {
    /// The key to the stored data.
    pub key: usize,

    /// The original template.
    pub template: String,

    /// The specificity of the template.
    pub specificity: usize,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<S> {
    /// The type of Node, and associated structure data.
    pub state: S,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a template.
    pub data: Option<NodeData>,

    pub static_children: Vec<Node<StaticState>>,
    pub dynamic_children: Vec<Node<DynamicState>>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Vec<Node<WildcardState>>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard: Option<Box<Node<EndWildcardState>>>,

    /// Flag indicating whether this node need optimization.
    /// During optimization, the shortcut flags are updated, and nodes sorted.
    pub needs_optimization: bool,
}

impl<S: Ord> Ord for Node<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state.cmp(&other.state)
    }
}

impl<S: Ord> PartialOrd for Node<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
