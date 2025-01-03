use super::state::{DynamicState, EndWildcardState, State, StaticState, WildcardState};
use crate::vec::SortedVec;
use std::cmp::Ordering;
use wayfind_storage::Storage;

pub trait Data: PartialEq + Eq {
    fn id(&self) -> Option<usize>;
    fn priority(&self) -> usize;
}

pub trait Config: PartialEq + Eq {
    type Data: Data;

    const DELIMITER: u8;
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<C: Config, S: State> {
    /// The type of Node, and associated structure data.
    pub state: S,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a route.
    pub data: Storage<C::Data>,

    pub static_children: SortedVec<Node<C, StaticState>>,
    pub dynamic_children: SortedVec<Node<C, DynamicState>>,
    pub dynamic_children_shortcut: bool,
    pub wildcard_children: SortedVec<Node<C, WildcardState>>,
    pub wildcard_children_shortcut: bool,
    pub end_wildcard_children: SortedVec<Node<C, EndWildcardState>>,

    /// Higher values indicate more specific matches.
    pub priority: usize,
    /// Flag indicating whether this node or its children need optimization.
    pub needs_optimization: bool,
}

impl<C: Config, S: State> PartialOrd for Node<C, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Config, S: State> Ord for Node<C, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.state.cmp(&other.state))
    }
}
