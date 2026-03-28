use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::node::flags::Flags;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod conflict;
mod delete;
mod display;
mod find;
pub(crate) mod flags;
mod insert;
mod optimize;
pub(crate) mod search;

#[derive(Clone, Debug)]
pub(crate) struct NodeData {
    /// The key to the stored data in the router's slab.
    pub key: usize,

    /// This node's template.
    pub template: Box<str>,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug)]
pub(crate) struct Node<S> {
    /// Node ID.
    pub id: usize,

    /// The node's type-specific state.
    pub state: S,
    /// Optional data associated with this node.
    pub data: Option<NodeData>,

    pub static_children: Vec<Node<StaticState>>,
    pub dynamic_children: Vec<Node<DynamicState>>,
    pub wildcard_children: Vec<Node<WildcardState>>,
    pub end_wildcard: Option<Box<EndWildcardState>>,

    /// State flags.
    pub flags: Flags,
    /// Minimum bytes of remaining path needed for any match through this node.
    pub shortest: usize,
    /// Maximum bytes of remaining path for any match through this node.
    pub longest: usize,
    /// Exact suffixes the remaining path must end with for any match through this node.
    pub tails: Box<[Box<[u8]>]>,
}

impl<S> Node<S> {
    /// Creates a new empty node.
    #[must_use]
    pub(crate) fn new(state: S) -> Self {
        Self {
            id: 0,

            state,
            data: None,

            static_children: Vec::new(),
            dynamic_children: Vec::new(),
            wildcard_children: Vec::new(),
            end_wildcard: None,

            flags: Flags::default(),
            shortest: usize::MAX,
            longest: 0,
            tails: Box::default(),
        }
    }
}
