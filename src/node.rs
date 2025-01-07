use std::sync::Arc;

use crate::{
    nodes::Nodes,
    state::{
        DynamicConstrainedState, DynamicState, EndWildcardConstrainedState, EndWildcardState,
        NodeState, StaticState, WildcardConstrainedState, WildcardState,
    },
};

mod delete;
mod display;
mod find;
mod insert;
mod optimize;
mod search;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeData<'r, T> {
    /// Data is stored inline.
    Inline {
        /// The associated data.
        data: T,

        /// The original template.
        template: &'r str,

        /// The number of slashes in the template.
        depth: usize,

        /// The length of the template.
        length: usize,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The associated data, shared.
        data: Arc<T>,

        /// The original template.
        template: &'r str,

        /// The expanded template.
        expanded: Arc<str>,

        /// The number of slashes in the expanded template.
        depth: usize,

        /// The length of the expanded template.
        length: usize,
    },
}

impl<T> NodeData<'_, T> {
    #[inline]
    pub fn data(&self) -> &T {
        match self {
            NodeData::Inline { data, .. } => data,
            NodeData::Shared { data, .. } => data.as_ref(),
        }
    }

    pub const fn template(&self) -> &str {
        match self {
            NodeData::Inline { template, .. } | NodeData::Shared { template, .. } => template,
        }
    }

    #[inline]
    pub fn expanded(&self) -> Option<&str> {
        match self {
            NodeData::Inline { .. } => None,
            NodeData::Shared { expanded, .. } => Some(expanded.as_ref()),
        }
    }

    pub const fn depth(&self) -> usize {
        match self {
            NodeData::Inline { depth, .. } | NodeData::Shared { depth, .. } => *depth,
        }
    }

    pub const fn length(&self) -> usize {
        match self {
            NodeData::Inline { length, .. } | NodeData::Shared { length, .. } => *length,
        }
    }
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<'r, T, S: NodeState> {
    /// The type of Node, and associated structure data.
    pub state: S,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a template.
    pub data: Option<NodeData<'r, T>>,

    pub static_children: Nodes<'r, T, StaticState>,
    pub dynamic_constrained_children: Nodes<'r, T, DynamicConstrainedState>,
    pub dynamic_children: Nodes<'r, T, DynamicState>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_constrained_children: Nodes<'r, T, WildcardConstrainedState>,
    pub wildcard_children: Nodes<'r, T, WildcardState>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_constrained_children: Nodes<'r, T, EndWildcardConstrainedState>,
    pub end_wildcard_children: Nodes<'r, T, EndWildcardState>,

    /// Flag indicating whether this node need optimization.
    /// During optimization, the shortcut flags are updated, and nodes sorted.
    pub needs_optimization: bool,
}
