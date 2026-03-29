use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::node::flags::Flags;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod conflict;
mod display;
pub(crate) mod flags;
mod insert;
mod optimize;
mod search;

/// Data stored at a node that matches a template.
#[derive(Clone, Debug)]
pub(crate) struct NodeData<T> {
    /// The associated data.
    pub data: T,

    /// This node's template.
    pub template: Box<str>,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug)]
pub(crate) struct Node<S, T> {
    /// The node's type-specific state.
    pub state: S,
    /// Optional data associated with this node.
    pub data: Option<NodeData<T>>,

    pub static_children: Vec<Node<StaticState, T>>,
    pub dynamic_children: Vec<Node<DynamicState, T>>,
    pub wildcard_children: Vec<Node<WildcardState, T>>,
    pub end_wildcard: Option<Box<Node<EndWildcardState, T>>>,

    /// State flags.
    pub flags: Flags,
    /// Minimum bytes of remaining path needed for any match through this node.
    pub shortest: usize,
    /// Maximum bytes of remaining path for any match through this node.
    pub longest: usize,
    /// Possible fixed suffixes the path must end with for any match through this node.
    pub tails: Box<[Box<[u8]>]>,
}

impl<S, T> Node<S, T> {
    /// Creates a new empty node.
    #[must_use]
    pub(crate) fn new(state: S) -> Self {
        Self {
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
