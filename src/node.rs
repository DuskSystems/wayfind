use std::sync::Arc;

use crate::{
    state::{DynamicState, EndWildcardState, NodeState, StaticState, WildcardState},
    vec::SortedNode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeData<'r, T> {
    /// Data is stored inline.
    Inline {
        /// The associated data.
        data: T,

        /// The original template.
        template: &'r str,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The associated data, shared.
        data: Arc<T>,

        /// The original template.
        template: &'r str,

        /// The expanded template.
        expanded: Arc<str>,
    },
}

impl<T> NodeData<'_, T> {
    pub const fn template(&self) -> &str {
        match self {
            NodeData::Inline { template, .. } | NodeData::Shared { template, .. } => template,
        }
    }

    pub fn priority(&self) -> usize {
        match self {
            NodeData::Inline { template, .. } => {
                template.len() + (template.bytes().filter(|&b| b == b'/').count() * 100)
            }
            NodeData::Shared { expanded, .. } => {
                expanded.len() + (expanded.bytes().filter(|&b| b == b'/').count() * 100)
            }
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

    pub static_children: SortedNode<'r, T, StaticState>,
    pub dynamic_children: SortedNode<'r, T, DynamicState>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: SortedNode<'r, T, WildcardState>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: SortedNode<'r, T, EndWildcardState>,

    /// Higher values indicate more specific matches.
    pub priority: usize,
    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}
