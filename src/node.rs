use alloc::string::String;

use crate::{
    nodes::Nodes,
    state::{DynamicState, EndWildcardState, NodeState, StaticState, WildcardState},
    storage::Key,
};

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

    /// The expanded template (if from optional group).
    pub expanded: Option<String>,

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

    pub static_children: Nodes<StaticState>,
    pub dynamic_children: Nodes<DynamicState>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: Nodes<WildcardState>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: Nodes<EndWildcardState>,

    /// Flag indicating whether this node need optimization.
    /// During optimization, the shortcut flags are updated, and nodes sorted.
    pub needs_optimization: bool,
}
