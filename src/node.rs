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

#[derive(Clone, Debug)]
pub struct NodeData {
    /// The key to the stored data in the router's slab.
    pub key: usize,

    /// This node's template.
    pub template: String,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Node<S> {
    /// The node's type-specific state.
    pub state: S,
    /// Optional data associated with this node.
    pub data: Option<NodeData>,

    pub static_children: Vec<Node<StaticState>>,
    pub dynamic_children: Vec<Node<DynamicState>>,
    pub wildcard_children: Vec<Node<WildcardState>>,
    pub end_wildcard: Option<Box<EndWildcardState>>,

    /// Whether this node has only static children (no dynamic, wildcard, or end wildcard).
    pub static_only: bool,
    /// Whether all dynamic children are full segments.
    pub dynamic_segment_only: bool,
    /// Whether all wildcard children are full segments.
    pub wildcard_segment_only: bool,
    /// Minimum bytes of remaining path needed for any match through this node.
    pub shortest: usize,
    /// Maximum bytes of remaining path for any match through this node.
    pub longest: usize,
    /// Possible fixed suffixes the path must end with for any match through this node.
    pub tails: Box<[Box<[u8]>]>,

    /// Whether this node needs optimization.
    pub needs_optimization: bool,
}

impl<S> Node<S> {
    /// Creates a new empty node.
    #[must_use]
    pub fn new(state: S) -> Self {
        Self {
            state,
            data: None,

            static_children: Vec::new(),
            dynamic_children: Vec::new(),
            wildcard_children: Vec::new(),
            end_wildcard: None,

            static_only: false,
            dynamic_segment_only: false,
            wildcard_segment_only: false,
            shortest: usize::MAX,
            longest: 0,
            tails: Box::default(),

            needs_optimization: false,
        }
    }
}
