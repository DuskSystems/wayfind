use core::cmp::Ordering;

use alloc::{string::String, vec::Vec};

use crate::{
    state::{DynamicState, EndWildcardState, NodeState, StaticState, WildcardState},
    storage::Key,
};

mod conflict;
mod delete;
mod display;
mod find;
mod insert;
mod optimize;
mod search;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData {
    /// The key to the stored data
    pub key: Key,

    /// The original template.
    pub template: String,

    /// The number of slashes in the template, or expanded template if exists.
    pub depth: usize,

    /// The length of the template, or expanded template if exists.
    pub length: usize,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<S: NodeState> {
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
    pub end_wildcard_children: Vec<Node<EndWildcardState>>,

    /// Flag indicating whether this node need optimization.
    /// During optimization, the shortcut flags are updated, and nodes sorted.
    pub needs_optimization: bool,
}

impl<S: NodeState> Ord for Node<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state.cmp(&other.state)
    }
}

impl<S: NodeState> PartialOrd for Node<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
