use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;

use crate::priority::Priority;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod conflict;
mod delete;
mod display;
mod find;
mod insert;
mod optimize;
mod search;

#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(core::mem::size_of::<Node<crate::state::RootState>>() == 168);
    assert!(core::mem::size_of::<Node<crate::state::StaticState>>() == 192);
    assert!(core::mem::size_of::<Node<crate::state::DynamicState>>() == 192);
    assert!(core::mem::size_of::<Node<crate::state::WildcardState>>() == 192);
    assert!(core::mem::size_of::<Node<crate::state::EndWildcardState>>() == 192);
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeData {
    /// The key to the stored data in the routers slab.
    pub key: usize,

    /// This nodes template.
    pub template: String,

    /// The priority of the template.
    pub priority: Priority,
}

/// Represents a node in the tree structure.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node<S> {
    /// The type of Node, and associated structure data.
    pub state: S,
    /// Optional data associated with this node.
    /// The presence of this data is needed to successfully match a template.
    pub data: Option<NodeData>,

    pub static_children: Vec<Node<StaticState>>,
    pub dynamic_children: Vec<Node<DynamicState>>,
    pub wildcard_children: Vec<Node<WildcardState>>,
    pub end_wildcard: Option<Box<Node<EndWildcardState>>>,

    /// Precomputed static suffixes for inline parameter matching.
    pub static_suffixes: Vec<Vec<u8>>,
    /// Whether all dynamic children are full segments, allowing for faster searching.
    pub dynamic_segment_only: bool,
    /// Whether all wildcard children are full segments, allowing for faster searching.
    pub wildcard_segment_only: bool,

    /// Flag indicating whether this node need optimization.
    /// During optimization, the shortcut flags are updated, priority calculated, and nodes sorted.
    pub needs_optimization: bool,
}

impl<S: Ord> Ord for Node<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state.cmp(&other.state)
    }
}

impl<S: Ord> PartialOrd for Node<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
