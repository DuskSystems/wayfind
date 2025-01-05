use std::sync::Arc;

use crate::{
    sorted::SortedNode,
    state::{
        DynamicConstrainedState, DynamicState, EndWildcardConstrainedState, EndWildcardState,
        NodeState, StaticState, WildcardConstrainedState, WildcardState,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeData<'r, T> {
    /// Data is stored inline.
    Inline {
        /// The associated data.
        data: T,

        /// The original template.
        template: &'r str,

        /// How 'specific' of a match this data is, based on the template complexity.
        specificity: usize,
    },

    /// Data is shared between 2 or more nodes.
    Shared {
        /// The associated data, shared.
        data: Arc<T>,

        /// The original template.
        template: &'r str,

        /// The expanded template.
        expanded: Arc<str>,

        /// How 'specific' of a match this data is, based on the expanded template complexity.
        specificity: usize,
    },
}

impl<T> NodeData<'_, T> {
    pub const fn template(&self) -> &str {
        match self {
            NodeData::Inline { template, .. } | NodeData::Shared { template, .. } => template,
        }
    }

    pub const fn specificity(&self) -> usize {
        match self {
            NodeData::Inline { specificity, .. } | NodeData::Shared { specificity, .. } => {
                *specificity
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
    pub dynamic_constrained_children: SortedNode<'r, T, DynamicConstrainedState>,
    pub dynamic_children: SortedNode<'r, T, DynamicState>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_constrained_children: SortedNode<'r, T, WildcardConstrainedState>,
    pub wildcard_children: SortedNode<'r, T, WildcardState>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_constrained_children: SortedNode<'r, T, EndWildcardConstrainedState>,
    pub end_wildcard_children: SortedNode<'r, T, EndWildcardState>,

    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}
