use super::{
    state::{DynamicState, EndWildcardState, State, StaticState, WildcardState},
    AuthorityData,
};
use crate::vec::SortedVec;
use std::cmp::Ordering;

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<'r, S: State> {
    /// The type of Node, and associated structure data.
    pub state: S,

    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Option<AuthorityData<'r>>,

    pub static_children: SortedVec<Node<'r, StaticState>>,
    pub dynamic_children: SortedVec<Node<'r, DynamicState>>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: SortedVec<Node<'r, WildcardState>>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: SortedVec<Node<'r, EndWildcardState>>,

    /// Higher values indicate more specific matches.
    pub priority: usize,
    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}

impl<S: State> PartialOrd for Node<'_, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State> Ord for Node<'_, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.state.cmp(&other.state))
    }
}
