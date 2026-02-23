use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod conflict;
mod delete;
mod display;
mod find;
mod insert;
mod optimize;
mod search;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeData {
    /// The key to the stored data in the router's slab.
    pub key: usize,

    /// This node's template.
    pub template: String,
}

/// Represents a node in the tree structure.
#[derive(Clone, Eq, PartialEq, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Node<S> {
    /// The node's type-specific state.
    pub state: S,
    /// Optional data associated with this node.
    pub data: Option<NodeData>,

    pub static_children: Vec<Node<StaticState>>,
    pub dynamic_children: Vec<Node<DynamicState>>,
    pub wildcard_children: Vec<Node<WildcardState>>,
    pub end_wildcard: Option<Box<Node<EndWildcardState>>>,

    /// Pre-computed static suffixes for inline parameter matching.
    pub static_suffixes: Vec<Vec<u8>>,
    /// Whether all dynamic children are full segments.
    pub dynamic_segment_only: bool,
    /// Whether all wildcard children are full segments.
    pub wildcard_segment_only: bool,

    /// Whether this node needs optimization.
    pub needs_optimization: bool,
}

impl<S> Node<S> {
    /// Creates a new empty node.
    #[must_use]
    pub const fn new(state: S) -> Self {
        Self {
            state,
            data: None,

            static_children: Vec::new(),
            dynamic_children: Vec::new(),
            wildcard_children: Vec::new(),
            end_wildcard: None,

            static_suffixes: Vec::new(),
            dynamic_segment_only: false,
            wildcard_segment_only: false,

            needs_optimization: false,
        }
    }
}
