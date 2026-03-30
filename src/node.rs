use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::node::bounds::Bounds;
use crate::node::flags::Flags;
use crate::node::suffixes::Suffixes;
use crate::node::tails::Tails;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod bounds;
mod conflict;
mod display;
pub(crate) mod flags;
mod insert;
mod optimize;
mod search;
pub(crate) use search::Search;
mod suffixes;
mod tails;

/// Data stored at a node that matches a template.
#[derive(Clone, Debug)]
pub(crate) struct Data<T> {
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
    pub data: Option<Data<T>>,

    pub static_children: Vec<Node<StaticState, T>>,
    pub dynamic_children: Vec<Node<DynamicState, T>>,
    pub wildcard_children: Vec<Node<WildcardState, T>>,
    pub end_wildcard: Option<Box<Node<EndWildcardState, T>>>,

    /// State flags.
    pub flags: Flags,
    /// Precomputed length bounds for pruning during search.
    pub bounds: Bounds,
    /// Possible fixed suffixes the path must end with for any match through this node.
    pub tails: Tails,
    /// Byte needles from static children.
    pub suffixes: Suffixes,
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
            bounds: Bounds::default(),
            tails: Tails::default(),
            suffixes: Suffixes::default(),
        }
    }
}
